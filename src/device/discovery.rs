use crate::device::identity::DeviceIdentity;
use hidapi::HidApi;

/// Phase 1a: Passive Enumeration
/// Discovers all connected HID devices and top-level collections via hidapi.
pub struct DeviceDiscovery;

impl DeviceDiscovery {
    pub fn enumerate_all() -> Vec<DeviceIdentity> {
        let Ok(api) = HidApi::new() else {
            return Vec::new();
        };

        let mut devices = Vec::new();

        for dev in api.device_list() {
            let vid = dev.vendor_id();
            let pid = dev.product_id();
            let is_cbg = vid == 0x258A && pid == 0x002A;

            let identity = DeviceIdentity {
                vendor_id: vid,
                product_id: pid,
                manufacturer: dev.manufacturer_string().map(|s| s.to_string()),
                product_name: dev.product_string().map(|s| s.to_string()),
                serial_number: dev.serial_number().map(|s| s.to_string()),
                usage_page: dev.usage_page(),
                usage: dev.usage(),
                interface_number: dev.interface_number(),
                path: dev.path().to_string_lossy().to_string(),
                is_cbg_k26: is_cbg,
            };

            devices.push(identity);
        }

        devices.sort_by(|a, b| {
            b.is_cbg_k26
                .cmp(&a.is_cbg_k26)
                .then_with(|| a.vendor_id.cmp(&b.vendor_id))
                .then_with(|| a.interface_number.cmp(&b.interface_number))
        });

        devices
    }
}
