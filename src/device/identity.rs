use serde::{Deserialize, Serialize};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::UI::Input::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: Option<String>,
    pub product_name: Option<String>,
    pub serial_number: Option<String>,
    pub usage_page: u16,
    pub usage: u16,
    pub interface_number: i32,
    pub path: String,
    pub is_cbg_k26: bool,
}

impl DeviceIdentity {
    pub const K26_VID: u16 = 0x258A;
    pub const K26_PID: u16 = 0x002A;

    pub fn is_target_k26(&self) -> bool {
        self.vendor_id == Self::K26_VID && self.product_id == Self::K26_PID
    }

    pub fn display_name(&self) -> String {
        if self.is_target_k26() {
            "Cosmic Byte Pandora CBG K26 (confirmed)".to_string()
        } else if let Some(ref prod) = self.product_name {
            prod.clone()
        } else if self.vendor_id != 0 {
            format!("HID Keyboard [VID 0x{:04X}: PID 0x{:04X}]", self.vendor_id, self.product_id)
        } else {
            "Internal / Unknown Keyboard".to_string()
        }
    }
}

pub struct RawDeviceResolver;

impl RawDeviceResolver {
    /// Queries the NT device path from a Windows Raw Input device handle (hDevice).
    pub fn get_device_path(h_device: HANDLE) -> Option<String> {
        unsafe {
            let mut size: u32 = 0;
            // First call with null buffer to query required buffer size
            let res = GetRawInputDeviceInfoW(
                Some(h_device),
                RIDI_DEVICENAME,
                None,
                &mut size,
            );

            if res == u32::MAX || size == 0 {
                return None;
            }

            let mut buffer: Vec<u16> = vec![0; size as usize];
            let res2 = GetRawInputDeviceInfoW(
                Some(h_device),
                RIDI_DEVICENAME,
                Some(buffer.as_mut_ptr() as *mut _),
                &mut size,
            );

            if res2 == u32::MAX {
                return None;
            }

            // Remove trailing null terminator if present
            let slice = if let Some(pos) = buffer.iter().position(|&c| c == 0) {
                &buffer[..pos]
            } else {
                &buffer[..]
            };

            Some(String::from_utf16_lossy(slice))
        }
    }

    /// Parses the Vendor ID (VID) and Product ID (PID) from an NT device path string.
    pub fn parse_vid_pid_from_path(path: &str) -> (u16, u16) {
        let upper = path.to_uppercase();

        let vid = if let Some(idx) = upper.find("VID_") {
            let hex_str = &upper[idx + 4..];
            let hex_slice: String = hex_str.chars().take(4).collect();
            u16::from_str_radix(&hex_slice, 16).unwrap_or(0)
        } else {
            0
        };

        let pid = if let Some(idx) = upper.find("PID_") {
            let hex_str = &upper[idx + 4..];
            let hex_slice: String = hex_str.chars().take(4).collect();
            u16::from_str_radix(&hex_slice, 16).unwrap_or(0)
        } else {
            0
        };

        (vid, pid)
    }

    /// Resolves full DeviceIdentity from Raw Input hDevice and known passive enumerated devices.
    pub fn resolve_from_handle(
        h_device: HANDLE,
        enumerated: &[DeviceIdentity],
    ) -> Option<DeviceIdentity> {
        let path = Self::get_device_path(h_device)?;
        let (vid, pid) = Self::parse_vid_pid_from_path(&path);

        // Try exact path match first
        if let Some(matched) = enumerated.iter().find(|d| d.path.eq_ignore_ascii_case(&path)) {
            return Some(matched.clone());
        }

        // Try VID & PID match second
        if vid != 0 && pid != 0 {
            if let Some(matched) = enumerated.iter().find(|d| d.vendor_id == vid && d.product_id == pid) {
                let mut customized = matched.clone();
                customized.path = path;
                return Some(customized);
            }
        }

        // Fallback: Unknown keyboard with parsed raw info
        let is_cbg = vid == DeviceIdentity::K26_VID && pid == DeviceIdentity::K26_PID;
        Some(DeviceIdentity {
            vendor_id: vid,
            product_id: pid,
            manufacturer: None,
            product_name: None,
            serial_number: None,
            usage_page: 0x01,
            usage: 0x06,
            interface_number: 0,
            path,
            is_cbg_k26: is_cbg,
        })
    }
}
