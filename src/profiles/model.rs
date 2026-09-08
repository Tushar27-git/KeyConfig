use crate::input::keys::VKey;
use crate::remap::rules::MappingTarget;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_readonly: bool,
    pub mappings: HashMap<VKey, MappingTarget>,
}

impl Profile {
    pub fn new_default() -> Self {
        Self {
            id: "default".to_string(),
            name: "Default".to_string(),
            description: "Factory standard layout with 1:1 pass-through.".to_string(),
            is_readonly: true,
            mappings: HashMap::new(),
        }
    }

    pub fn new_gaming() -> Self {
        let mut mappings = HashMap::new();
        mappings.insert(VKey::WinLeft, MappingTarget::Block);
        mappings.insert(VKey::CapsLock, MappingTarget::Key(VKey::ControlLeft));

        Self {
            id: "gaming".to_string(),
            name: "Gaming".to_string(),
            description: "Disables Windows key to prevent accidental task-switches; CapsLock remapped to Ctrl."
                .to_string(),
            is_readonly: false,
            mappings,
        }
    }

    pub fn new_work() -> Self {
        let mut mappings = HashMap::new();
        mappings.insert(VKey::CapsLock, MappingTarget::Key(VKey::Escape));

        Self {
            id: "work".to_string(),
            name: "Work / Dev".to_string(),
            description: "Optimized for development: CapsLock remapped to Escape.".to_string(),
            is_readonly: false,
            mappings,
        }
    }
}
