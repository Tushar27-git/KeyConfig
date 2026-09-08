use crate::input::keys::VKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemapRule {
    KeyToKey { source: VKey, target: VKey },
    BlockKey { source: VKey },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MappingTarget {
    Key(VKey),
    Block,
}

impl MappingTarget {
    pub fn label(&self) -> &'static str {
        match self {
            MappingTarget::Key(k) => k.label(),
            MappingTarget::Block => "✕",
        }
    }

    pub fn display_target(&self) -> String {
        match self {
            MappingTarget::Key(k) => format!("Remapped → {:?}", k),
            MappingTarget::Block => "[BLOCKED]".to_string(),
        }
    }
}
