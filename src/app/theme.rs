use egui::{Color32, CornerRadius, Stroke, Visuals};

pub struct Theme;

impl Theme {
    // Background tokens
    pub const BG_BASE: Color32 = Color32::from_rgb(11, 13, 17);
    pub const BG_SURFACE: Color32 = Color32::from_rgb(18, 21, 27);
    pub const BG_ELEVATED: Color32 = Color32::from_rgb(26, 30, 39);
    pub const BG_KEYCAP: Color32 = Color32::from_rgb(30, 35, 45);

    // Border tokens
    pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(34, 39, 51);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(59, 130, 246);

    // Text tokens
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(241, 245, 249);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(148, 163, 184);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(100, 116, 139);

    // Accent tokens
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(6, 182, 212);
    pub const ACCENT_GREEN: Color32 = Color32::from_rgb(16, 185, 129);
    pub const ACCENT_AMBER: Color32 = Color32::from_rgb(245, 158, 11);
    pub const ACCENT_RED: Color32 = Color32::from_rgb(239, 68, 68);

    pub fn apply(ctx: &egui::Context) {
        let mut visuals = Visuals::dark();

        visuals.panel_fill = Self::BG_BASE;
        visuals.window_fill = Self::BG_SURFACE;
        visuals.override_text_color = Some(Self::TEXT_PRIMARY);

        // Widgets styling
        visuals.widgets.noninteractive.bg_fill = Self::BG_SURFACE;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Self::BORDER_SUBTLE);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_SECONDARY);
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(3);

        visuals.widgets.inactive.bg_fill = Self::BG_ELEVATED;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Self::BORDER_SUBTLE);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, Self::TEXT_PRIMARY);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(3);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(38, 44, 58);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, Self::BORDER_FOCUS);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(3);

        visuals.widgets.active.bg_fill = Color32::from_rgb(45, 52, 70);
        visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, Self::ACCENT_CYAN);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        visuals.widgets.active.corner_radius = CornerRadius::same(3);

        visuals.selection.bg_fill = Color32::from_rgba_premultiplied(6, 182, 212, 60);
        visuals.selection.stroke = Stroke::new(1.0_f32, Self::ACCENT_CYAN);

        ctx.set_visuals(visuals);
    }
}
