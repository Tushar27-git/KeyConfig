use crate::device::manager::DeviceManager;
use crate::diagnostics::monitor::EventMonitor;
use crate::diagnostics::rate::{RateCalculator, RateStats};
use crate::input::keys::VKey;
use crate::input::normalize::{InputEvent, KeyState};
use crate::profiles::model::Profile;
use crate::profiles::store::ProfileStore;
use crate::remap::rules::MappingTarget;
use crate::startup::{ShortcutManager, StartupLaunchMode, StartupManager, StartupStatus};
use crossbeam_channel::Receiver;
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavView {
    Keyboard,
    Profiles,
    Remap,
    Diagnostics,
    Device,
    Settings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CapturedKeyInfo {
    pub vkey: VKey,
    pub scan_code: u32,
    pub raw_vk: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CaptureState {
    #[default]
    Idle,
    ListeningForTarget,
    CapturedTarget(CapturedKeyInfo),
    ListeningForReplacement,
    CapturedReplacement(CapturedKeyInfo),
}

impl CaptureState {
    pub fn is_listening(&self) -> bool {
        matches!(self, Self::ListeningForTarget | Self::ListeningForReplacement)
    }
}

pub struct AppState {
    pub active_view: NavView,
    pub device_manager: DeviceManager,
    pub storage: ProfileStore,
    pub profiles: HashMap<String, Profile>,
    pub active_profile_id: String,
    pub active_profile_arc: Arc<RwLock<Profile>>,
    pub pressed_keys: HashSet<VKey>,
    pub selected_key: Option<VKey>,
    pub capture_state: CaptureState,
    pub capture_active: Arc<AtomicBool>,
    pub show_manual_picker: bool,
    pub test_input_text: String,
    pub swap_target_key: Option<VKey>,
    pub matrix_new_source: Option<VKey>,
    pub matrix_new_target: Option<VKey>,
    pub event_log: EventMonitor,
    pub rate_calc: RateCalculator,
    pub rate_stats: RateStats,
    pub hotplug_trigger: Arc<AtomicBool>,
    pub stop_flag: Arc<AtomicBool>,
    pub event_rx: Receiver<InputEvent>,
    pub device_rx: Receiver<String>,
    pub status_message: Option<(String, std::time::Instant)>,
    pub start_minimized_pending: bool,
    pub startup_status: StartupStatus,
    pub selected_startup_mode: StartupLaunchMode,
    pub shortcut_installed: bool,
}

impl AppState {
    pub fn new(
        event_rx: Receiver<InputEvent>,
        device_rx: Receiver<String>,
        capture_active: Arc<AtomicBool>,
        active_profile_arc: Arc<RwLock<Profile>>,
    ) -> Self {
        let storage = ProfileStore::new();
        let profiles = storage.load_all();
        let active_profile_id = "default".to_string();

        let initial_profile = profiles
            .get(&active_profile_id)
            .cloned()
            .unwrap_or_else(Profile::new_default);
        // CRITICAL: Synchronize the loaded profile to the shared Arc so the hook thread immediately has it!
        *active_profile_arc.write() = initial_profile;

        let hotplug_trigger = Arc::new(AtomicBool::new(false));
        let stop_flag = Arc::new(AtomicBool::new(false));

        // Start background hotplug thread
        DeviceManager::start_hotplug_monitor(hotplug_trigger.clone(), stop_flag.clone());

        let startup_status = StartupManager::query_status().unwrap_or(StartupStatus {
            enabled: false,
            command: None,
            launch_mode: StartupLaunchMode::Normal,
            is_current_exe: false,
            detected_exe: StartupManager::get_recommended_executable().ok(),
        });
        let selected_startup_mode = startup_status.launch_mode;

        Self {
            active_view: NavView::Keyboard,
            device_manager: DeviceManager::new(),
            storage,
            profiles,
            active_profile_id,
            active_profile_arc,
            pressed_keys: HashSet::new(),
            selected_key: Some(VKey::Backslash),
            capture_state: CaptureState::Idle,
            capture_active,
            show_manual_picker: false,
            test_input_text: String::new(),
            swap_target_key: Some(VKey::Backspace),
            matrix_new_source: Some(VKey::Backslash),
            matrix_new_target: Some(VKey::Backspace),
            event_log: EventMonitor::new(1000),
            rate_calc: RateCalculator::new(128),
            rate_stats: RateStats::default(),
            hotplug_trigger,
            stop_flag,
            event_rx,
            device_rx,
            status_message: None,
            start_minimized_pending: false,
            startup_status,
            selected_startup_mode,
            shortcut_installed: ShortcutManager::is_installed(),
        }
    }

    pub fn start_listening_target(&mut self) {
        self.capture_state = CaptureState::ListeningForTarget;
        self.capture_active.store(true, Ordering::SeqCst);
        self.set_status("Listening: Press physical key for target...");
    }

    pub fn start_listening_replacement(&mut self) {
        self.capture_state = CaptureState::ListeningForReplacement;
        self.capture_active.store(true, Ordering::SeqCst);
        self.set_status("Listening: Press physical key for replacement...");
    }

    pub fn cancel_capture(&mut self) {
        self.capture_state = CaptureState::Idle;
        self.capture_active.store(false, Ordering::SeqCst);
    }

    pub fn confirm_captured_replacement(&mut self) {
        if let CaptureState::CapturedReplacement(info) = self.capture_state {
            if let Some(target) = self.selected_key {
                self.set_mapping(target, MappingTarget::Key(info.vkey));
                self.set_status(&format!("Remapped {:?} → {:?}", target, info.vkey));
            }
        }
        self.capture_state = CaptureState::Idle;
        self.capture_active.store(false, Ordering::SeqCst);
    }

    pub fn confirm_captured_target(&mut self) {
        if let CaptureState::CapturedTarget(info) = self.capture_state {
            self.selected_key = Some(info.vkey);
            self.set_status(&format!("Target confirmed: {:?}", info.vkey));
        }
        self.capture_state = CaptureState::Idle;
        self.capture_active.store(false, Ordering::SeqCst);
    }

    /// Process incoming events from the Raw Input and Low-Level Hook channels.
    /// Returns true if any new input or device activity occurred.
    pub fn process_incoming_events(&mut self) -> bool {
        let mut had_activity = false;

        // 1. Process Raw Input device path events (identify-by-keystroke)
        while let Ok(path) = self.device_rx.try_recv() {
            had_activity = true;
            if self.device_manager.on_keystroke_path(&path) {
                let label = self.device_manager.active_keyboard_label();
                self.set_status(&format!("Active keyboard identified: {}", label));
            }
        }

        // 2. Process keystroke events from the hook
        let mut count = 0;
        while let Ok(event) = self.event_rx.try_recv() {
            had_activity = true;
            count += 1;
            if count > 256 {
                break;
            }

            // Capture mode state machine
            if !event.is_self_injected() && event.state == KeyState::Down {
                match self.capture_state {
                    CaptureState::ListeningForTarget => {
                        let info = CapturedKeyInfo {
                            vkey: event.vkey,
                            scan_code: event.scan_code,
                            raw_vk: event.raw_vk,
                        };
                        self.selected_key = Some(event.vkey);
                        self.capture_state = CaptureState::CapturedTarget(info);
                        self.capture_active.store(false, Ordering::SeqCst);
                        self.set_status(&format!(
                            "Target key captured: {:?} (Scan 0x{:02X})",
                            event.vkey, event.scan_code
                        ));
                    }
                    CaptureState::ListeningForReplacement => {
                        let info = CapturedKeyInfo {
                            vkey: event.vkey,
                            scan_code: event.scan_code,
                            raw_vk: event.raw_vk,
                        };
                        // Immediately apply remapping as soon as the physical key is pressed!
                        if let Some(target) = self.selected_key {
                            self.set_mapping(target, MappingTarget::Key(event.vkey));
                            self.set_status(&format!(
                                "✓ Remapped {:?} → {:?} (Scan 0x{:02X})",
                                target, event.vkey, event.scan_code
                            ));
                        }
                        self.capture_state = CaptureState::CapturedReplacement(info);
                        self.capture_active.store(false, Ordering::SeqCst);
                    }
                    _ => {}
                }
            }

            // Update visual keyboard pressed state
            if !event.is_self_injected() {
                match event.state {
                    KeyState::Down => {
                        self.pressed_keys.insert(event.vkey);
                        self.rate_calc.record_event(event.instant_micros);
                    }
                    KeyState::Up => {
                        self.pressed_keys.remove(&event.vkey);
                    }
                }
            }

            // Push into live monitor
            self.event_log.push(event);
        }

        // Recompute observed rate statistics only when new keystrokes arrive
        if count > 0 {
            self.rate_stats = self.rate_calc.compute_stats();
        }

        // 3. Process hotplug trigger (WM_INPUT_DEVICE_CHANGE or background poll)
        if self.hotplug_trigger.swap(false, Ordering::SeqCst) {
            had_activity = true;
            let diffs = self.device_manager.refresh();
            for diff in diffs {
                match diff {
                    crate::device::DeviceDiff::Connected(dev) => {
                        self.set_status(&format!("Keyboard attached: {}", dev.display_name()));
                    }
                    crate::device::DeviceDiff::Disconnected(_) => {
                        self.set_status("Keyboard disconnected");
                    }
                    crate::device::DeviceDiff::NoChange => {}
                }
            }
        }

        had_activity
    }

    pub fn set_active_profile(&mut self, id: &str) {
        if let Some(prof) = self.profiles.get(id).cloned() {
            self.active_profile_id = id.to_string();
            *self.active_profile_arc.write() = prof;
            self.set_status(&format!("Activated profile '{}'", id));
        }
    }

    pub fn set_mapping(&mut self, source: VKey, target: MappingTarget) {
        if let Some(profile) = self.profiles.get_mut(&self.active_profile_id) {
            let desc = match target {
                MappingTarget::Key(k) => format!("{:?} → {:?}", source, k),
                MappingTarget::Block => format!("{:?} [BLOCKED]", source),
            };
            profile.mappings.insert(source, target);
            *self.active_profile_arc.write() = profile.clone();
            let _ = self.storage.save_profile(profile);
            self.set_status(&format!("✓ Applied to system: {}", desc));
        }
    }

    pub fn remove_mapping(&mut self, source: VKey) {
        if let Some(profile) = self.profiles.get_mut(&self.active_profile_id) {
            profile.mappings.remove(&source);
            *self.active_profile_arc.write() = profile.clone();
            let _ = self.storage.save_profile(profile);
            self.set_status(&format!("✓ Reset mapping for {:?} (1:1 pass-through)", source));
        }
    }

    pub fn swap_mappings(&mut self, key_a: VKey, key_b: VKey) {
        if key_a == key_b {
            return;
        }
        if let Some(profile) = self.profiles.get_mut(&self.active_profile_id) {
            profile.mappings.insert(key_a, MappingTarget::Key(key_b));
            profile.mappings.insert(key_b, MappingTarget::Key(key_a));
            *self.active_profile_arc.write() = profile.clone();
            let _ = self.storage.save_profile(profile);
            self.set_status(&format!("✓ Swapped on system: {:?} ↔ {:?}", key_a, key_b));
        }
    }

    pub fn clear_all_mappings(&mut self) {
        if let Some(profile) = self.profiles.get_mut(&self.active_profile_id) {
            profile.mappings.clear();
            *self.active_profile_arc.write() = profile.clone();
            let _ = self.storage.save_profile(profile);
            self.set_status("✓ Cleared all mappings: All physical keys now pass through 1:1");
        }
    }

    pub fn save_and_apply_to_system(&mut self) {
        if let Some(profile) = self.profiles.get(&self.active_profile_id) {
            *self.active_profile_arc.write() = profile.clone();
            let _ = self.storage.save_profile(profile);
            self.set_status(&format!(
                "✓ APPLIED TO SYSTEM: Profile '{}' active with {} mapping(s)",
                profile.name,
                profile.mappings.len()
            ));
        }
    }

    pub fn create_profile(&mut self, name: &str) -> String {
        let id = name.to_lowercase().replace(' ', "-");
        let profile = Profile {
            id: id.clone(),
            name: name.to_string(),
            description: "Custom user profile.".to_string(),
            is_readonly: false,
            mappings: HashMap::new(),
        };

        let _ = self.storage.save_profile(&profile);
        self.profiles.insert(id.clone(), profile);
        self.set_active_profile(&id);
        id
    }

    pub fn delete_profile(&mut self, id: &str) {
        if let Some(prof) = self.profiles.get(id) {
            if prof.is_readonly {
                self.set_status("Cannot delete read-only default profile.");
                return;
            }
        }
        let _ = self.storage.delete_profile(id);
        self.profiles.remove(id);
        if self.active_profile_id == id {
            self.set_active_profile("default");
        }
    }

    pub fn refresh_startup_status(&mut self) {
        if let Ok(status) = StartupManager::query_status() {
            self.selected_startup_mode = status.launch_mode;
            self.startup_status = status;
        }
    }

    pub fn enable_autostart(&mut self) {
        match StartupManager::enable(self.selected_startup_mode) {
            Ok(_) => {
                self.refresh_startup_status();
                self.set_status(&format!(
                    "✓ Auto-startup enabled ({})",
                    self.selected_startup_mode.as_str()
                ));
            }
            Err(e) => {
                self.set_status(&format!("Failed to enable auto-startup: {:?}", e));
            }
        }
    }

    pub fn disable_autostart(&mut self) {
        match StartupManager::disable() {
            Ok(_) => {
                self.refresh_startup_status();
                self.set_status("✓ Auto-startup disabled (removed from Windows Run)");
            }
            Err(e) => {
                self.set_status(&format!("Failed to disable auto-startup: {:?}", e));
            }
        }
    }

    pub fn install_shortcut(&mut self) {
        if let Ok(exe) = StartupManager::get_recommended_executable() {
            let working_dir = exe.parent().unwrap_or(std::path::Path::new("."));
            let icon_candidate = working_dir.join("assets").join("theasus.ico");
            let icon_path = if icon_candidate.is_file() {
                Some(icon_candidate.as_path())
            } else {
                None
            };
            match ShortcutManager::install(&exe, working_dir, icon_path) {
                Ok(_) => {
                    self.shortcut_installed = true;
                    self.set_status("✓ Added to Windows Start Menu (Searchable by pressing Win key)");
                }
                Err(e) => {
                    self.set_status(&format!("Failed to install Start Menu shortcut: {:?}", e));
                }
            }
        }
    }

    pub fn uninstall_shortcut(&mut self) {
        match ShortcutManager::uninstall() {
            Ok(_) => {
                self.shortcut_installed = false;
                self.set_status("✓ Removed Start Menu shortcuts & App Paths");
            }
            Err(e) => {
                self.set_status(&format!("Failed to remove Start Menu shortcuts: {:?}", e));
            }
        }
    }

    pub fn set_status(&mut self, msg: &str) {
        self.status_message = Some((msg.to_string(), std::time::Instant::now()));
    }

    pub fn current_status(&self) -> Option<&str> {
        if let Some((ref msg, instant)) = self.status_message {
            if instant.elapsed() < std::time::Duration::from_secs(5) {
                return Some(msg);
            }
        }
        None
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }
}
