pub mod views;

use crate::app::state::{AppState, NavView};
use crate::app::theme::Theme;
use egui::{RichText, Ui};
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
        if !state.pressed_keys.is_empty() || state.capture_mode {
            ctx.request_repaint();
        }

        // Top Hardware Status Bar
        egui::TopBottomPanel::top("status_bar")
            .frame(egui::Frame::NONE.fill(Theme::BG_SURFACE).stroke(egui::Stroke::new(1.0_f32, Theme::BORDER_SUBTLE)).inner_margin(8.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Some(active) = state.device_manager.active_keyboard() {
                        ui.label(RichText::new("● ACTIVE").color(Theme::ACCENT_GREEN).strong().small());
                        let label = state.device_manager.active_keyboard_label();
                        ui.label(RichText::new(label).strong());
                        ui.label(RichText::new(format!("[VID: 0x{:04X} | PID: 0x{:04X}]", active.vendor_id, active.product_id)).color(Theme::TEXT_MUTED).monospace().small());
                    } else if state.device_manager.is_k26_detected() {
                        ui.label(RichText::new("● DETECTED").color(Theme::ACCENT_CYAN).strong().small());
                        ui.label(RichText::new("Cosmic Byte Pandora CBG K26 (Press any key)").strong());
                        ui.label(RichText::new("[VID: 0x258A | PID: 0x002A]").color(Theme::TEXT_MUTED).monospace().small());
                    } else {
                        ui.label(RichText::new("○ WAITING...").color(Theme::ACCENT_AMBER).strong().small());
                        ui.label(RichText::new("No Keyboard Keystroke Detected Yet").color(Theme::TEXT_SECONDARY));
                    }

                    ui.separator();

                    let active_name = state.profiles.get(&state.active_profile_id)
                        .map(|p| p.name.as_str())
                        .unwrap_or("Default");
                    ui.label(RichText::new(format!("Profile: {}", active_name)).color(Theme::ACCENT_CYAN).strong());

                    if let Some(msg) = state.current_status() {
                        ui.separator();
                        ui.label(RichText::new(msg).color(Theme::ACCENT_AMBER).small());
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new("v0.1.0-auto-detect").color(Theme::TEXT_MUTED).small().monospace());
                    });
                });
            });

        // Left Navigation Sidebar
        egui::SidePanel::left("nav_sidebar")
            .resizable(false)
            .exact_width(170.0)
            .frame(egui::Frame::NONE.fill(Theme::BG_SURFACE).stroke(egui::Stroke::new(1.0_f32, Theme::BORDER_SUBTLE)).inner_margin(8.0))
            .show(ctx, |ui| {
                ui.label(RichText::new("⚡ THEASUS").color(Theme::ACCENT_AMBER).strong());
                ui.label(RichText::new("HARDWARE CONTROLLER").color(Theme::TEXT_MUTED).size(9.0));
                ui.add_space(8.0);

                self.nav_button(ui, state, NavView::Keyboard, "⌨  Keyboard");
                self.nav_button(ui, state, NavView::Profiles, "📁 Profiles");
                self.nav_button(ui, state, NavView::Remap, "🔀 Remap");
                self.nav_button(ui, state, NavView::Diagnostics, "📊 Diagnostics");
                self.nav_button(ui, state, NavView::Device, "🔌 Device");
                self.nav_button(ui, state, NavView::Settings, "⚙  Settings");

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label(RichText::new("Raw Input hDevice Engine").color(Theme::TEXT_MUTED).small());
                });
            });

        // Main Central Workspace
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Theme::BG_BASE).inner_margin(16.0))
            .show(ctx, |ui| {
                match state.active_view {
                    NavView::Keyboard => views::keyboard_view::show(ui, state),
                    NavView::Profiles => views::profiles_view::show(ui, state, &mut self.profiles_view_state),
                    NavView::Remap => views::remap_view::show(ui, state),
                    NavView::Diagnostics => views::diagnostics_view::show(ui, state),
                    NavView::Device => views::device_view::show(ui, state),
                    NavView::Settings => views::settings_view::show(ui, state),
                }
            });
    }

    fn nav_button(&self, ui: &mut Ui, state: &mut AppState, view: NavView, label: &str) {
        let is_active = state.active_view == view;
        let text = if is_active {
            RichText::new(label).color(Theme::ACCENT_CYAN).strong()
        } else {
            RichText::new(label).color(Theme::TEXT_PRIMARY)
        };

        let response = ui.add_sized([ui.available_width(), 32.0], egui::Button::new(text));
        if response.clicked() {
            state.active_view = view;
        }
    }
}
