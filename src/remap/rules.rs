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
