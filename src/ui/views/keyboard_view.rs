use crate::app::state::AppState;
use crate::app::theme::Theme;
use crate::keyboard_visual::KeyboardRenderer;
use crate::remap::rules::MappingTarget;
use egui::{RichText, Ui};

pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        // Workspace Header
        ui.horizontal(|ui| {
            ui.heading(RichText::new("Keyboard Workspace").size(18.0).color(Theme::TEXT_PRIMARY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let profile_name = state.profiles.get(&state.active_profile_id)
                    .map(|p| p.name.as_str())
                    .unwrap_or("Default");
                ui.label(RichText::new(format!("Active Profile: {}", profile_name)).color(Theme::ACCENT_CYAN).monospace());
            });
        });
        ui.add_space(8.0);

        // Visual Keyboard Area (CBG K26 68-Key ANSI Mechanical)
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                let active_profile = state.profiles.get(&state.active_profile_id)
                    .cloned()
                    .unwrap_or_else(crate::profiles::model::Profile::new_default);

                let renderer = KeyboardRenderer::new(
                    &state.pressed_keys,
                    state.selected_key,
                    &active_profile,
                );

                if let Some(clicked) = renderer.render(ui) {
                    state.selected_key = Some(clicked);
                    state.capture_mode = false;
                }
                ui.add_space(8.0);
            });
        });

        ui.add_space(12.0);

        // Selected Key Quick-Remap Card
        if let Some(selected_vkey) = state.selected_key {
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("Selected Key: {:?}", selected_vkey)).strong().size(15.0));
                    ui.label(RichText::new(format!("(VK 0x{:02X})", selected_vkey.to_vk())).color(Theme::TEXT_MUTED).monospace());

                    let active_profile = state.profiles.get(&state.active_profile_id);
                    let current_target = active_profile.and_then(|p| p.mappings.get(&selected_vkey));

                    ui.separator();

                    match current_target {
                        Some(MappingTarget::Key(dest)) => {
                            ui.label(RichText::new(format!("Mapped to: {:?}", dest)).color(Theme::ACCENT_CYAN).strong());
                            if ui.button("Reset to Default").clicked() {
                                state.remove_mapping(selected_vkey);
                            }
                        }
                        Some(MappingTarget::Block) => {
                            ui.label(RichText::new("Blocked").color(Theme::ACCENT_RED).strong());
                            if ui.button("Unblock").clicked() {
                                state.remove_mapping(selected_vkey);
                            }
                        }
                        None => {
                            ui.label(RichText::new("Assignment: Default (1:1 Pass-through)").color(Theme::TEXT_SECONDARY));
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if state.capture_mode {
                            ui.label(RichText::new("⚡ Press any physical key...").color(Theme::ACCENT_AMBER).strong());
                            if ui.button("Cancel").clicked() {
                                state.capture_mode = false;
                            }
                        } else if ui.button("Capture Physical Key").clicked() {
                            state.capture_mode = true;
                        }

                        if ui.button("Block Key").clicked() {
                            state.set_mapping(selected_vkey, MappingTarget::Block);
                        }
                    });
                });
            });
        }

        ui.add_space(8.0);

        // Live status tip
        ui.horizontal(|ui| {
            ui.label(RichText::new("Auto-Detection Active:").color(Theme::ACCENT_GREEN).small().strong());
            ui.label(RichText::new("Typing on any keyboard automatically attributes keystrokes to its hardware device handle.").color(Theme::TEXT_SECONDARY).small());
        });
    });
}
