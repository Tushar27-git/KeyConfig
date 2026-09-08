use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownDeviceEntry {
    pub vendor_id: u16,
    pub vendor_id_hex: String,
    pub product_id: u16,
    pub product_id_hex: String,
    pub manufacturer: String,
    pub product_name: String,
    pub model_label: String,
    pub confirmed: bool,
    pub verified_at: String,
    pub notes: String,
}

pub struct KnownDevicesDatabase {
    entries: Vec<KnownDeviceEntry>,
}

impl KnownDevicesDatabase {
    pub fn load() -> Self {
        // Try looking in config/known_devices.json relative to exe or cwd
        let possible_paths = [
            Path::new("config/known_devices.json"),
            Path::new("../config/known_devices.json"),
        ];

        for path in possible_paths {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(entries) = serde_json::from_str::<Vec<KnownDeviceEntry>>(&content) {
                        return Self { entries };
                    }
                }
            }
        }

        Self {
            entries: Vec::new(),
        }
    }

    /// Matches a VID and PID against the verified database.
    /// Strictly adheres to the addendum:
    /// - If matched and confirmed: "Cosmic Byte Pandora CBG K26 (confirmed)"
    /// - Otherwise: "Unknown keyboard — showing raw HID identity" (never guesses).
    pub fn resolve_label(&self, vid: u16, pid: u16) -> Option<String> {
        self.entries
            .iter()
            .find(|e| e.vendor_id == vid && e.product_id == pid && e.confirmed)
            .map(|e| e.model_label.clone())
    }

    pub fn is_confirmed_cbg_k26(&self, vid: u16, pid: u16) -> bool {
        vid == 0x258A && pid == 0x002A
    }
}
