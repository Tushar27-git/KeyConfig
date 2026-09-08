use crate::device::capabilities::DeviceCapabilities;
use crate::device::discovery::DeviceDiscovery;
use crate::device::identity::{DeviceIdentity, RawDeviceResolver};
use crate::device::known_devices::KnownDevicesDatabase;
use crate::device::reconnect::{DeviceDiff, ReconnectHandler};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct DeviceManager {
    detected_keyboards: Vec<DeviceIdentity>,
    active_keyboard: Option<DeviceIdentity>,
    selected_device_index: usize,
    known_db: KnownDevicesDatabase,
    capabilities: DeviceCapabilities,
}

impl DeviceManager {
    pub fn new() -> Self {
        let known_db = KnownDevicesDatabase::load();
        let detected = DeviceDiscovery::enumerate_all();
        let is_k26 = detected.iter().any(|d| d.is_cbg_k26);
        let capabilities = if is_k26 {
            DeviceCapabilities::for_cbg_k26()
        } else {
            DeviceCapabilities::generic_keyboard()
        };

        let initial_active = detected.iter().find(|d| d.is_cbg_k26).cloned();

        Self {
            detected_keyboards: detected,
            active_keyboard: initial_active,
            selected_device_index: 0,
            known_db,
            capabilities,
        }
    }

    /// Called on every Raw Input keystroke with the physical device path.
    /// This is the primary identify-by-keystroke primitive!
    pub fn on_keystroke_path(&mut self, path: &str) -> bool {
        // Try exact match in detected keyboards
        let identified = if let Some(matched) = self.detected_keyboards.iter().find(|d| d.path.eq_ignore_ascii_case(path)) {
            matched.clone()
        } else {
            let (vid, pid) = RawDeviceResolver::parse_vid_pid_from_path(path);
            let is_cbg = vid == DeviceIdentity::K26_VID && pid == DeviceIdentity::K26_PID;
            DeviceIdentity {
                vendor_id: vid,
                product_id: pid,
                manufacturer: None,
                product_name: None,
                serial_number: None,
                usage_page: 0x01,
                usage: 0x06,
                interface_number: 0,
                path: path.to_string(),
                is_cbg_k26: is_cbg,
            }
        };

        let changed = match &self.active_keyboard {
            Some(current) => current.path != identified.path,
            None => true,
        };

        if changed {
            tracing::info!(
                "Active keyboard identified by keystroke: VID 0x{:04X}, PID 0x{:04X}, Path: {}",
                identified.vendor_id,
                identified.product_id,
                identified.path
            );
            if identified.is_cbg_k26 {
                self.capabilities = DeviceCapabilities::for_cbg_k26();
            }
            self.active_keyboard = Some(identified);
            return true;
        }

        false
    }

    pub fn refresh(&mut self) -> Vec<DeviceDiff> {
        let (current, diffs) = ReconnectHandler::diff(&self.detected_keyboards);
        self.detected_keyboards = current;

        if let Some(ref active) = self.active_keyboard {
            if !self.detected_keyboards.iter().any(|d| d.path == active.path) {
                tracing::warn!("Active keyboard disconnected: {}", active.path);
                self.active_keyboard = None;
            }
        }

        let is_k26 = self.detected_keyboards.iter().any(|d| d.is_cbg_k26);
        self.capabilities = if is_k26 {
            DeviceCapabilities::for_cbg_k26()
        } else {
            DeviceCapabilities::generic_keyboard()
        };

        diffs
    }

    pub fn active_keyboard(&self) -> Option<&DeviceIdentity> {
        self.active_keyboard.as_ref()
    }

    pub fn active_keyboard_label(&self) -> String {
        if let Some(ref active) = self.active_keyboard {
            if let Some(label) = self.known_db.resolve_label(active.vendor_id, active.product_id) {
                label
            } else if active.is_cbg_k26 {
                "Cosmic Byte Pandora CBG K26 (confirmed)".to_string()
            } else if active.vendor_id != 0 {
                format!(
                    "Unknown keyboard — showing raw HID identity (VID: 0x{:04X}, PID: 0x{:04X})",
                    active.vendor_id, active.product_id
                )
            } else {
                "Internal Laptop / PS/2 Keyboard".to_string()
            }
        } else {
            "No active keystroke detected yet — press any key to identify".to_string()
        }
    }

    pub fn is_k26_active(&self) -> bool {
        self.active_keyboard.as_ref().map(|d| d.is_cbg_k26).unwrap_or(false)
    }

    pub fn is_k26_detected(&self) -> bool {
        self.detected_keyboards.iter().any(|d| d.is_cbg_k26)
    }

    pub fn detected_keyboards(&self) -> &[DeviceIdentity] {
        &self.detected_keyboards
    }

    pub fn capabilities(&self) -> &DeviceCapabilities {
        &self.capabilities
    }

    pub fn selected_device(&self) -> Option<&DeviceIdentity> {
        self.detected_keyboards.get(self.selected_device_index)
    }

    pub fn select_device(&mut self, index: usize) {
        if index < self.detected_keyboards.len() {
            self.selected_device_index = index;
        }
    }

    pub fn start_hotplug_monitor(
        trigger: Arc<AtomicBool>,
        stop_flag: Arc<AtomicBool>,
    ) -> std::thread::JoinHandle<()> {
        std::thread::Builder::new()
            .name("device-hotplug-monitor".to_string())
            .spawn(move || {
                while !stop_flag.load(Ordering::Relaxed) {
                    std::thread::sleep(Duration::from_millis(1500));
                    trigger.store(true, Ordering::SeqCst);
                }
            })
            .expect("Failed to spawn hotplug monitor thread")
    }
}
