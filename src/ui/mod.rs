pub mod views;

use crate::app::state::{AppState, NavView};
use crate::app::theme::Theme;
use egui::{Color32, CornerRadius, Pos2, Rect, RichText, Ui, Vec2};
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

        // Top Hardware Status Bar with Sleek Studio Header
        egui::TopBottomPanel::top("status_bar")
            .frame(
                egui::Frame::NONE
                    .fill(Theme::BG_SURFACE)
                    .stroke(egui::Stroke::new(1.0_f32, Theme::BORDER_SUBTLE))
                    .inner_margin(egui::Margin::symmetric(16, 10)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Some(active) = state.device_manager.active_keyboard() {
                        ui.label(
                            RichText::new("● ONLINE")
                                .color(Theme::ACCENT_GREEN)
                                .size(11.0)
                                .strong(),
                        );
                        let label = state.device_manager.active_keyboard_label();
                        ui.label(
                            RichText::new(label)
                                .color(Theme::TEXT_PRIMARY)
                                .strong()
                                .size(13.0),
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
                            RichText::new("● DETECTED")
                                .color(Theme::ACCENT_BLUE)
                                .size(11.0)
                                .strong(),
                        );
                        ui.label(
                            RichText::new("Cosmic Byte Pandora CBG K26")
                                .color(Theme::TEXT_PRIMARY)
                                .strong()
                                .size(13.0),
                        );
                        ui.label(
                            RichText::new("• Press any physical key to activate")
                                .color(Theme::ACCENT_AMBER)
                                .small(),
                        );
                    } else {
                        ui.label(
                            RichText::new("○ STANDBY")
                                .color(Theme::ACCENT_AMBER)
                                .size(11.0)
                                .strong(),
                        );
                        ui.label(
                            RichText::new("Waiting for physical keystroke...")
                                .color(Theme::TEXT_MUTED)
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
                            .small(),
                    );
                    ui.label(
                        RichText::new(active_name.to_uppercase())
                            .color(Theme::ACCENT_VIOLET)
                            .strong()
                            .size(12.0),
                    );

                    if let Some(msg) = state.current_status() {
                        ui.separator();
                        ui.label(
                            RichText::new(msg)
                                .color(Theme::ACCENT_AMBER)
                                .small(),
                        );
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new("THEASUS STUDIO v0.1.0")
                                .color(Theme::TEXT_MUTED)
                                .small(),
                        );
                    });
                });
            });

        // Left Navigation Sidebar
        egui::SidePanel::left("nav_sidebar")
            .resizable(false)
            .exact_width(185.0)
            .frame(
                egui::Frame::NONE
                    .fill(Theme::BG_SURFACE)
                    .stroke(egui::Stroke::new(1.0_f32, Theme::BORDER_SUBTLE))
                    .inner_margin(12.0),
            )
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("THEASUS")
                            .color(Theme::TEXT_PRIMARY)
                            .size(17.0)
                            .strong(),
                    );
                    ui.label(
                        RichText::new("STUDIO")
                            .color(Theme::ACCENT_BLUE)
                            .size(11.0)
                            .strong(),
                    );
                });
                ui.label(
                    RichText::new("KEYBOARD CONTROL CENTER")
                        .color(Theme::TEXT_MUTED)
                        .size(9.0),
                );
                ui.add_space(14.0);

                self.nav_button(ui, state, NavView::Keyboard, "⌨  Keyboard");
                self.nav_button(ui, state, NavView::Remap, "🔀  Remap Engine");
                self.nav_button(ui, state, NavView::Profiles, "📁  Profiles");
                self.nav_button(ui, state, NavView::Diagnostics, "📊  Diagnostics");
                self.nav_button(ui, state, NavView::Device, "🔌  Device Info");
                self.nav_button(ui, state, NavView::Settings, "⚙  Settings");

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label(
                        RichText::new("● Native Win32 Hook Active")
                            .color(Theme::ACCENT_GREEN)
                            .size(9.5),
                    );
                });
            });

        // Main Central Workspace
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Theme::BG_BASE).inner_margin(18.0))
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
        let button_size = Vec2::new(ui.available_width(), 36.0);
        let (rect, response) = ui.allocate_exact_size(button_size, egui::Sense::click());

        if response.clicked() {
            state.active_view = view;
        }

        let painter = ui.painter_at(rect);

        // Hover or Active background with smooth modern 6px rounded pill
        if is_active {
            painter.rect_filled(
                rect,
                CornerRadius::same(6),
                Color32::from_rgb(0x1C, 0x22, 0x36),
            );

            // Sleek electric cyan left indicator bar
            let indicator_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + 3.0, rect.min.y + 6.0),
                Vec2::new(3.5, rect.height() - 12.0),
            );
            painter.rect_filled(
                indicator_rect,
                CornerRadius::same(2),
                Theme::ACCENT_BLUE,
            );
        } else if response.hovered() {
            painter.rect_filled(
                rect,
                CornerRadius::same(6),
                Color32::from_rgb(0x18, 0x1C, 0x2C),
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
            Pos2::new(rect.min.x + 14.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(13.0),
            text_color,
        );

        ui.add_space(3.0);
    }
}
