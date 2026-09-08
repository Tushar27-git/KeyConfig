use crate::input::keys::VKey;

#[derive(Clone, Debug)]
pub struct KeyDefinition {
    pub vkey: VKey,
    pub width_u: f32,
    pub secondary_label: Option<&'static str>,
}

impl KeyDefinition {
    pub fn new(vkey: VKey, width_u: f32) -> Self {
        Self {
            vkey,
            width_u,
            secondary_label: None,
        }
    }

    pub fn with_sub(vkey: VKey, width_u: f32, sub: &'static str) -> Self {
        Self {
            vkey,
            width_u,
            secondary_label: Some(sub),
        }
    }
}

/// Physical layout definition of the Cosmic Byte Pandora CBG K26 (68-key / 65% ANSI).
pub fn k26_physical_layout() -> Vec<Vec<KeyDefinition>> {
    use VKey::*;
    vec![
        // Row 0: Top Number Row + Nav
        vec![
            KeyDefinition::with_sub(Escape, 1.0, "`"),
            KeyDefinition::with_sub(Num1, 1.0, "!"),
            KeyDefinition::with_sub(Num2, 1.0, "@"),
            KeyDefinition::with_sub(Num3, 1.0, "#"),
            KeyDefinition::with_sub(Num4, 1.0, "$"),
            KeyDefinition::with_sub(Num5, 1.0, "%"),
            KeyDefinition::with_sub(Num6, 1.0, "^"),
            KeyDefinition::with_sub(Num7, 1.0, "&"),
            KeyDefinition::with_sub(Num8, 1.0, "*"),
            KeyDefinition::with_sub(Num9, 1.0, "("),
            KeyDefinition::with_sub(Num0, 1.0, ")"),
            KeyDefinition::with_sub(Minus, 1.0, "_"),
            KeyDefinition::with_sub(Equal, 1.0, "+"),
            KeyDefinition::new(Backspace, 2.0),
            KeyDefinition::new(Backquote, 1.0),
        ],
        // Row 1: QWERTY + Nav
        vec![
            KeyDefinition::new(Tab, 1.5),
            KeyDefinition::new(KeyQ, 1.0),
            KeyDefinition::new(KeyW, 1.0),
            KeyDefinition::new(KeyE, 1.0),
            KeyDefinition::new(KeyR, 1.0),
            KeyDefinition::new(KeyT, 1.0),
            KeyDefinition::new(KeyY, 1.0),
            KeyDefinition::new(KeyU, 1.0),
            KeyDefinition::new(KeyI, 1.0),
            KeyDefinition::new(KeyO, 1.0),
            KeyDefinition::new(KeyP, 1.0),
            KeyDefinition::with_sub(BracketLeft, 1.0, "{"),
            KeyDefinition::with_sub(BracketRight, 1.0, "}"),
            KeyDefinition::with_sub(Backslash, 1.5, "|"),
            KeyDefinition::new(Delete, 1.0),
        ],
        // Row 2: Home Row + Nav
        vec![
            KeyDefinition::new(CapsLock, 1.75),
            KeyDefinition::new(KeyA, 1.0),
            KeyDefinition::new(KeyS, 1.0),
            KeyDefinition::new(KeyD, 1.0),
            KeyDefinition::new(KeyF, 1.0),
            KeyDefinition::new(KeyG, 1.0),
            KeyDefinition::new(KeyH, 1.0),
            KeyDefinition::new(KeyJ, 1.0),
            KeyDefinition::new(KeyK, 1.0),
            KeyDefinition::new(KeyL, 1.0),
            KeyDefinition::with_sub(Semicolon, 1.0, ":"),
            KeyDefinition::with_sub(Quote, 1.0, "\""),
            KeyDefinition::new(Enter, 2.25),
            KeyDefinition::new(PageUp, 1.0),
        ],
        // Row 3: Shift Row + Arrows + Nav
        vec![
            KeyDefinition::new(ShiftLeft, 2.25),
            KeyDefinition::new(KeyZ, 1.0),
            KeyDefinition::new(KeyX, 1.0),
            KeyDefinition::new(KeyC, 1.0),
            KeyDefinition::new(KeyV, 1.0),
            KeyDefinition::new(KeyB, 1.0),
            KeyDefinition::new(KeyN, 1.0),
            KeyDefinition::new(KeyM, 1.0),
            KeyDefinition::with_sub(Comma, 1.0, "<"),
            KeyDefinition::with_sub(Period, 1.0, ">"),
            KeyDefinition::with_sub(Slash, 1.0, "?"),
            KeyDefinition::new(ShiftRight, 1.75),
            KeyDefinition::new(ArrowUp, 1.0),
            KeyDefinition::new(PageDown, 1.0),
        ],
        // Row 4: Bottom Modifiers Row + Arrows
        vec![
            KeyDefinition::new(ControlLeft, 1.25),
            KeyDefinition::new(WinLeft, 1.25),
            KeyDefinition::new(AltLeft, 1.25),
            KeyDefinition::new(Space, 6.25),
            KeyDefinition::new(AltRight, 1.25),
            KeyDefinition::new(ControlRight, 1.25),
            KeyDefinition::new(ArrowLeft, 1.0),
            KeyDefinition::new(ArrowDown, 1.0),
            KeyDefinition::new(ArrowRight, 1.0),
        ],
    ]
}
