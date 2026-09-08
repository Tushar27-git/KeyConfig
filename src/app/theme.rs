use egui::{Color32, CornerRadius, Stroke, Visuals};

pub struct Theme;

impl Theme {
    // Background tokens (Retro Precision Terminal)
    pub const BG_BASE: Color32 = Color32::from_rgb(0x0A, 0x0A, 0x0F);
    pub const BG_SURFACE: Color32 = Color32::from_rgb(0x12, 0x12, 0x1B);
    pub const BG_ELEVATED: Color32 = Color32::from_rgb(0x18, 0x18, 0x24);
    pub const BG_KEYCAP: Color32 = Color32::from_rgb(0x16, 0x16, 0x22);

    // Border tokens
    pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(0x24, 0x24, 0x33);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(0x3D, 0xA9, 0xFC);

    // Text tokens
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0xE4, 0xE4, 0xEC);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(0x9E, 0x9E, 0xB0);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x6B, 0x6B, 0x80);

    // Accent tokens
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(0x3D, 0xA9, 0xFC); // Electric Blue #3DA9FC
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(0x3D, 0xA9, 0xFC); // Alias for compatibility
    pub const ACCENT_VIOLET: Color32 = Color32::from_rgb(0xA3, 0x74, 0xFF); // Violet #A374FF
    pub const ACCENT_GREEN: Color32 = Color32::from_rgb(0x3D, 0xFC, 0xA0); // Success #3DFCA0
    pub const ACCENT_AMBER: Color32 = Color32::from_rgb(0xFC, 0xA1, 0x3D); // Warning #FCA13D
    pub const ACCENT_RED: Color32 = Color32::from_rgb(0xFC, 0x3D, 0x5A); // Danger #FC3D5A

    pub fn apply(ctx: &egui::Context) {
        let mut visuals = Visuals::dark();

        visuals.panel_fill = Self::BG_BASE;
        visuals.window_fill = Self::BG_SURFACE;
        visuals.override_text_color = Some(Self::TEXT_PRIMARY);

        // Sharp corners (0-2px) per retro precision-terminal spec
        let sharp_radius = CornerRadius::same(1);

        // Widgets styling
        visuals.widgets.noninteractive.bg_fill = Self::BG_SURFACE;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Self::BORDER_SUBTLE);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_SECONDARY);
        visuals.widgets.noninteractive.corner_radius = sharp_radius;

        visuals.widgets.inactive.bg_fill = Self::BG_ELEVATED;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Self::BORDER_SUBTLE);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_PRIMARY);
        visuals.widgets.inactive.corner_radius = sharp_radius;

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(0x22, 0x22, 0x32);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, Self::ACCENT_VIOLET);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        visuals.widgets.hovered.corner_radius = sharp_radius;

        visuals.widgets.active.bg_fill = Color32::from_rgb(0x2C, 0x28, 0x44);
        visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, Self::ACCENT_BLUE);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        visuals.widgets.active.corner_radius = sharp_radius;

        visuals.selection.bg_fill = Color32::from_rgba_premultiplied(0x3D, 0xA9, 0xFC, 45);
        visuals.selection.stroke = Stroke::new(1.0_f32, Self::ACCENT_BLUE);

        ctx.set_visuals(visuals);
    }
}
