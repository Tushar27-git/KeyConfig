use egui::{Color32, CornerRadius, Stroke, Visuals};

pub struct Theme;

impl Theme {
    // Background tokens (Sleek Modern Cyberpunk)
    pub const BG_BASE: Color32 = Color32::from_rgb(0x0C, 0x0D, 0x14);
    pub const BG_SURFACE: Color32 = Color32::from_rgb(0x13, 0x14, 0x1F);
    pub const BG_ELEVATED: Color32 = Color32::from_rgb(0x1C, 0x1E, 0x2C);
    pub const BG_KEYCAP: Color32 = Color32::from_rgb(0x19, 0x1B, 0x28);

    // Border tokens
    pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(0x28, 0x2A, 0x3D);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(0x3D, 0xA9, 0xFC);

    // Text tokens
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0xEE, 0xEE, 0xF6);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(0xA2, 0xA4, 0xB8);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x6E, 0x70, 0x86);

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

        // Refined modern corner radius (4px)
        let modern_radius = CornerRadius::same(4);

        // Widgets styling
        visuals.widgets.noninteractive.bg_fill = Self::BG_SURFACE;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Self::BORDER_SUBTLE);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_SECONDARY);
        visuals.widgets.noninteractive.corner_radius = modern_radius;

        visuals.widgets.inactive.bg_fill = Self::BG_ELEVATED;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Self::BORDER_SUBTLE);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_PRIMARY);
        visuals.widgets.inactive.corner_radius = modern_radius;

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(0x28, 0x2A, 0x3E);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, Self::ACCENT_VIOLET);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        visuals.widgets.hovered.corner_radius = modern_radius;

        visuals.widgets.active.bg_fill = Color32::from_rgb(0x32, 0x2C, 0x50);
        visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, Self::ACCENT_BLUE);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        visuals.widgets.active.corner_radius = modern_radius;

        visuals.selection.bg_fill = Color32::from_rgba_premultiplied(0x3D, 0xA9, 0xFC, 45);
        visuals.selection.stroke = Stroke::new(1.0_f32, Self::ACCENT_BLUE);

        ctx.set_visuals(visuals);
    }
}
