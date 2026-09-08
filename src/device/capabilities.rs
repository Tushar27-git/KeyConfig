use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityStatus {
    Confirmed,
    Probable,
    Unknown,
    Unsupported,
}

impl CapabilityStatus {
    pub fn label(&self) -> &'static str {
        match self {
            CapabilityStatus::Confirmed => "CONFIRMED (HOST)",
            CapabilityStatus::Probable => "PROBABLE",
            CapabilityStatus::Unknown => "UNKNOWN / UNVERIFIED",
            CapabilityStatus::Unsupported => "NOT EXPOSED BY DEVICE",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    pub software_remap: CapabilityStatus,
    pub hardware_remap: CapabilityStatus,
    pub polling_rate_control: CapabilityStatus,
    pub rgb_control: CapabilityStatus,
    pub macro_support: CapabilityStatus,
    pub onboard_profiles: CapabilityStatus,
    pub firmware_info: CapabilityStatus,
}

impl DeviceCapabilities {
    /// Factory for Cosmic Byte Pandora CBG K26 capabilities.
    /// Strictly adheres to the Master Build Prompt and Hardware Reality Rule:
    /// Never fake capabilities or promise features the hardware protocol has not proven.
    pub fn for_cbg_k26() -> Self {
        Self {
            software_remap: CapabilityStatus::Confirmed,
            hardware_remap: CapabilityStatus::Unknown,
            polling_rate_control: CapabilityStatus::Unsupported,
            rgb_control: CapabilityStatus::Unknown,
            macro_support: CapabilityStatus::Unknown,
            onboard_profiles: CapabilityStatus::Unknown,
            firmware_info: CapabilityStatus::Probable,
        }
    }

    pub fn generic_keyboard() -> Self {
        Self {
            software_remap: CapabilityStatus::Confirmed,
            hardware_remap: CapabilityStatus::Unknown,
            polling_rate_control: CapabilityStatus::Unsupported,
            rgb_control: CapabilityStatus::Unsupported,
            macro_support: CapabilityStatus::Unsupported,
            onboard_profiles: CapabilityStatus::Unsupported,
            firmware_info: CapabilityStatus::Unknown,
        }
    }
}
