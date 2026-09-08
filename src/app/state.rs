use crate::device::manager::DeviceManager;
use crate::diagnostics::monitor::EventMonitor;
use crate::diagnostics::rate::{RateCalculator, RateStats};
use crate::input::keys::VKey;
use crate::input::normalize::{InputEvent, KeyState};
use crate::profiles::model::Profile;
use crate::profiles::store::ProfileStore;
use crate::remap::rules::MappingTarget;
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
    pub event_log: EventMonitor,
    pub rate_calc: RateCalculator,
    pub rate_stats: RateStats,
    pub hotplug_trigger: Arc<AtomicBool>,
    pub stop_flag: Arc<AtomicBool>,
    pub event_rx: Receiver<InputEvent>,
    pub device_rx: Receiver<String>,
    pub status_message: Option<(String, std::time::Instant)>,
}

impl AppState {
    pub fn new(
        event_rx: Receiver<InputEvent>,
        device_rx: Receiver<String>,
        capture_active: Arc<AtomicBool>,
    ) -> Self {
        let storage = ProfileStore::new();
        let profiles = storage.load_all();
        let active_profile_id = "default".to_string();

        let initial_profile = profiles
            .get(&active_profile_id)
            .cloned()
            .unwrap_or_else(Profile::new_default);
        let active_profile_arc = Arc::new(RwLock::new(initial_profile));

        let hotplug_trigger = Arc::new(AtomicBool::new(false));
        let stop_flag = Arc::new(AtomicBool::new(false));

        // Start background hotplug thread
        DeviceManager::start_hotplug_monitor(hotplug_trigger.clone(), stop_flag.clone());

        Self {
            active_view: NavView::Keyboard,
            device_manager: DeviceManager::new(),
            storage,
            profiles,
            active_profile_id,
            active_profile_arc,
            pressed_keys: HashSet::new(),
            selected_key: Some(VKey::CapsLock),
            capture_state: CaptureState::Idle,
            capture_active,
            show_manual_picker: false,
            event_log: EventMonitor::new(1000),
            rate_calc: RateCalculator::new(128),
            rate_stats: RateStats::default(),
            hotplug_trigger,
            stop_flag,
            event_rx,
            device_rx,
            status_message: None,
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
    pub fn process_incoming_events(&mut self) {
        // 1. Process Raw Input device path events (identify-by-keystroke)
        while let Ok(path) = self.device_rx.try_recv() {
            if self.device_manager.on_keystroke_path(&path) {
                let label = self.device_manager.active_keyboard_label();
                self.set_status(&format!("Active keyboard identified: {}", label));
            }
        }

        // 2. Process keystroke events from the hook
        let mut count = 0;
        while let Ok(event) = self.event_rx.try_recv() {
            count += 1;
            if count > 256 {
                break;
            }

            // Capture mode state machine
            if !event.is_injected() && event.state == KeyState::Down {
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
                        self.capture_state = CaptureState::CapturedReplacement(info);
                        self.capture_active.store(false, Ordering::SeqCst);
                        self.set_status(&format!(
                            "Replacement key captured: {:?} (Scan 0x{:02X})",
                            event.vkey, event.scan_code
                        ));
                    }
                    _ => {}
                }
            }

            // Update visual keyboard pressed state
            if !event.is_injected() {
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

        // Recompute observed rate statistics
        self.rate_stats = self.rate_calc.compute_stats();

        // 3. Process hotplug trigger (WM_INPUT_DEVICE_CHANGE or background poll)
        if self.hotplug_trigger.swap(false, Ordering::SeqCst) {
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
            if profile.is_readonly {
                self.set_status("Cannot edit read-only 'Default' profile. Switch to Gaming/Work or create a new profile.");
                return;
            }

            profile.mappings.insert(source, target);
            *self.active_profile_arc.write() = profile.clone();
            let _ = self.storage.save_profile(profile);
        }
    }

    pub fn remove_mapping(&mut self, source: VKey) {
        if let Some(profile) = self.profiles.get_mut(&self.active_profile_id) {
            if profile.is_readonly {
                self.set_status("Cannot edit read-only 'Default' profile.");
                return;
            }

            profile.mappings.remove(&source);
            *self.active_profile_arc.write() = profile.clone();
            let _ = self.storage.save_profile(profile);
            self.set_status(&format!("Reset mapping for {:?}", source));
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
