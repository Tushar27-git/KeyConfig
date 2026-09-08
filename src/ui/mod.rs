pub mod views;

use crate::app::state::{AppState, NavView};
use crate::app::theme::Theme;
use egui::{Color32, CornerRadius, Pos2, Rect, RichText, Stroke, Ui, Vec2};
use views::ProfilesViewState;

pub struct MainWindow {
    profiles_view_state: ProfilesViewState,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            profiles_view_state: ProfilesViewState::default(),
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, state: &mut AppState) {
        if !state.pressed_keys.is_empty() || state.capture_state.is_listening() {
            ctx.request_repaint();
        }

        // Top Hardware Status Bar with Retro Accent Underline
        egui::TopBottomPanel::top("status_bar")
            .frame(
                egui::Frame::NONE
                    .fill(Theme::BG_SURFACE)
                    .stroke(egui::Stroke::new(1.0_f32, Theme::BORDER_SUBTLE))
                    .inner_margin(egui::Margin::symmetric(12, 8)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Some(active) = state.device_manager.active_keyboard() {
                        ui.label(
                            RichText::new("● ACTIVE:")
                                .color(Theme::ACCENT_GREEN)
                                .monospace()
                                .small(),
                        );
                        let label = state.device_manager.active_keyboard_label();
                        ui.label(
                            RichText::new(label)
                                .color(Theme::TEXT_PRIMARY)
                                .strong()
                                .monospace(),
                        );
                        ui.label(
                            RichText::new(format!(
                                "[VID: 0x{:04X} | PID: 0x{:04X}]",
                                active.vendor_id, active.product_id
                            ))
                            .color(Theme::TEXT_MUTED)
                            .monospace()
                            .small(),
                        );
                    } else if state.device_manager.is_k26_detected() {
                        ui.label(
                            RichText::new("● DETECTED:")
                                .color(Theme::ACCENT_BLUE)
                                .monospace()
                                .small(),
                        );
                        ui.label(
                            RichText::new("Cosmic Byte Pandora CBG K26")
                                .strong()
                                .monospace(),
                        );
                        ui.label(
                            RichText::new("[Press any key to activate]")
                                .color(Theme::ACCENT_AMBER)
                                .monospace()
                                .small(),
                        );
                    } else {
                        ui.label(
                            RichText::new("○ STANDBY:")
                                .color(Theme::ACCENT_AMBER)
                                .monospace()
                                .small(),
                        );
                        ui.label(
                            RichText::new("Waiting for hardware keystroke...")
                                .color(Theme::TEXT_MUTED)
                                .monospace()
                                .small(),
                        );
                    }

                    ui.separator();

                    let active_name = state
                        .profiles
                        .get(&state.active_profile_id)
                        .map(|p| p.name.as_str())
                        .unwrap_or("Default");
                    ui.label(
                        RichText::new("PROFILE:")
                            .color(Theme::TEXT_MUTED)
                            .monospace()
                            .small(),
                    );
                    ui.label(
                        RichText::new(active_name.to_uppercase())
                            .color(Theme::ACCENT_VIOLET)
                            .strong()
                            .monospace(),
                    );

                    if let Some(msg) = state.current_status() {
                        ui.separator();
                        ui.label(
                            RichText::new(msg)
                                .color(Theme::ACCENT_AMBER)
                                .small()
                                .monospace(),
                        );
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new("THEASUS v0.1.0")
                                .color(Theme::TEXT_MUTED)
                                .small()
                                .monospace(),
                        );
                    });
                });

                // Accent colored underline strip
                ui.add_space(4.0);
                let (strip_rect, _) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), 1.5),
                    egui::Sense::hover(),
                );
                ui.painter().rect_filled(
                    strip_rect,
                    CornerRadius::same(0),
                    Color32::from_rgb(0x3D, 0xA9, 0xFC),
                );
            });

        // Left Navigation Sidebar
        egui::SidePanel::left("nav_sidebar")
            .resizable(false)
            .exact_width(175.0)
            .frame(
                egui::Frame::NONE
                    .fill(Theme::BG_SURFACE)
                    .stroke(egui::Stroke::new(1.0_f32, Theme::BORDER_SUBTLE))
                    .inner_margin(8.0),
            )
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.label(
                    RichText::new("⚡ THEASUS")
                        .color(Theme::ACCENT_AMBER)
                        .strong()
                        .monospace(),
                );
                ui.label(
                    RichText::new("HARDWARE CONTROLLER")
                        .color(Theme::TEXT_MUTED)
                        .size(9.0)
                        .monospace(),
                );
                ui.add_space(12.0);

                self.nav_button(ui, state, NavView::Keyboard, "⌨  KEYBOARD");
                self.nav_button(ui, state, NavView::Profiles, "📁 PROFILES");
                self.nav_button(ui, state, NavView::Remap, "🔀 REMAP");
                self.nav_button(ui, state, NavView::Diagnostics, "📊 DIAGNOSTICS");
                self.nav_button(ui, state, NavView::Device, "🔌 DEVICE");
                self.nav_button(ui, state, NavView::Settings, "⚙  SETTINGS");

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label(
                        RichText::new("PRECISION TERMINAL")
                            .color(Theme::TEXT_MUTED)
                            .small()
                            .monospace(),
                    );
                });
            });

        // Main Central Workspace
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Theme::BG_BASE).inner_margin(16.0))
            .show(ctx, |ui| {
                match state.active_view {
                    NavView::Keyboard => views::keyboard_view::show(ui, state),
                    NavView::Profiles => {
                        views::profiles_view::show(ui, state, &mut self.profiles_view_state)
                    }
                    NavView::Remap => views::remap_view::show(ui, state),
                    NavView::Diagnostics => views::diagnostics_view::show(ui, state),
                    NavView::Device => views::device_view::show(ui, state),
                    NavView::Settings => views::settings_view::show(ui, state),
                }
            });
    }

    fn nav_button(&self, ui: &mut Ui, state: &mut AppState, view: NavView, label: &str) {
        let is_active = state.active_view == view;
        let button_size = Vec2::new(ui.available_width(), 34.0);
        let (rect, response) = ui.allocate_exact_size(button_size, egui::Sense::click());

        if response.clicked() {
            state.active_view = view;
        }

        let painter = ui.painter_at(rect);

        // Hover or Active background
        if is_active {
            painter.rect_filled(
                rect,
                CornerRadius::same(1),
                Color32::from_rgb(0x18, 0x22, 0x36),
            );

            // 2px solid electric blue glowing left-edge indicator bar
            let indicator_rect = Rect::from_min_size(rect.min, Vec2::new(3.0, rect.height()));
            painter.rect_filled(
                indicator_rect,
                CornerRadius::same(1),
                Theme::ACCENT_BLUE,
            );

            // Subtle glow outside indicator
            painter.rect_stroke(
                indicator_rect.expand(1.0),
                CornerRadius::same(1),
                Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(0x3D, 0xA9, 0xFC, 80)),
                egui::StrokeKind::Outside,
            );
        } else if response.hovered() {
            painter.rect_filled(
                rect,
                CornerRadius::same(1),
                Color32::from_rgb(0x18, 0x18, 0x24),
            );
        }

        let text_color = if is_active {
            Theme::ACCENT_BLUE
        } else if response.hovered() {
            Theme::TEXT_PRIMARY
        } else {
            Theme::TEXT_SECONDARY
        };

        painter.text(
            Pos2::new(rect.min.x + 12.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::monospace(11.5),
            text_color,
        );

        ui.add_space(2.0);
    }
}
