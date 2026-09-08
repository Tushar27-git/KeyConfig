use egui::{Color32, CornerRadius, Stroke, Visuals};

pub struct Theme;

impl Theme {
    // Background tokens (Modern Premium Studio)
    pub const BG_BASE: Color32 = Color32::from_rgb(0x0A, 0x0C, 0x14);
    pub const BG_SURFACE: Color32 = Color32::from_rgb(0x12, 0x15, 0x22);
    pub const BG_ELEVATED: Color32 = Color32::from_rgb(0x1A, 0x1E, 0x30);
    pub const BG_KEYCAP: Color32 = Color32::from_rgb(0x18, 0x1B, 0x2C);

    // Border tokens
    pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(0x25, 0x29, 0x40);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(0x00, 0xD2, 0xFF);

    // Text tokens
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0xF0, 0xF2, 0xFA);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(0xA6, 0xAC, 0xCD);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x69, 0x70, 0x8D);

    // Accent tokens
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(0x00, 0xD2, 0xFF); // Electric Neon Cyan #00D2FF
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(0x00, 0xD2, 0xFF); // Neon Cyan
    pub const ACCENT_VIOLET: Color32 = Color32::from_rgb(0x8B, 0x5C, 0xF6); // Royal Violet #8B5CF6
    pub const ACCENT_GREEN: Color32 = Color32::from_rgb(0x10, 0xB9, 0x81); // Vibrant Emerald #10B981
    pub const ACCENT_AMBER: Color32 = Color32::from_rgb(0xF5, 0x9E, 0x0B); // Golden Amber #F59E0B
    pub const ACCENT_RED: Color32 = Color32::from_rgb(0xEF, 0x44, 0x44); // Crimson #EF4444

    pub fn apply(ctx: &egui::Context) {
        let mut visuals = Visuals::dark();

        visuals.panel_fill = Self::BG_BASE;
        visuals.window_fill = Self::BG_SURFACE;
        visuals.override_text_color = Some(Self::TEXT_PRIMARY);

        // Refined modern corner radius (6px)
        let modern_radius = CornerRadius::same(6);

        // Widgets styling
        visuals.widgets.noninteractive.bg_fill = Self::BG_SURFACE;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Self::BORDER_SUBTLE);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_SECONDARY);
        visuals.widgets.noninteractive.corner_radius = modern_radius;

        visuals.widgets.inactive.bg_fill = Self::BG_ELEVATED;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Self::BORDER_SUBTLE);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_PRIMARY);
        visuals.widgets.inactive.corner_radius = modern_radius;

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(0x24, 0x2A, 0x42);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, Self::ACCENT_VIOLET);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        visuals.widgets.hovered.corner_radius = modern_radius;

        visuals.widgets.active.bg_fill = Color32::from_rgb(0x2B, 0x33, 0x54);
        visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, Self::ACCENT_BLUE);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        visuals.widgets.active.corner_radius = modern_radius;

        visuals.selection.bg_fill = Color32::from_rgba_premultiplied(0x00, 0xD2, 0xFF, 40);
        visuals.selection.stroke = Stroke::new(1.0_f32, Self::ACCENT_BLUE);

        ctx.set_visuals(visuals);
    }
}
