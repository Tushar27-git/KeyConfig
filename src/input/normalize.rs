use crate::input::keys::VKey;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyState {
    Down,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputOrigin {
    Physical,
    Injected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemapAction {
    PassThrough,
    Remapped { target: VKey },
    Blocked,
    SelfInjected,
}

/// Normalized keyboard event produced by the Windows input pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputEvent {
    pub timestamp: DateTime<Local>,
    pub instant_micros: u64,
    pub vkey: VKey,
    pub raw_vk: u16,
    pub scan_code: u32,
    pub state: KeyState,
    pub origin: InputOrigin,
    pub action: RemapAction,
    pub device_handle_raw: usize, // hDevice raw address for cross-referencing
}

impl InputEvent {
    pub fn is_injected(&self) -> bool {
        self.origin == InputOrigin::Injected
    }
}
