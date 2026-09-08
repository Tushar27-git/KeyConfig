use crate::app::state::AppState;
use crate::app::theme::Theme;
use egui::{RichText, Ui};

pub struct ProfilesViewState {
    pub new_profile_name: String,
}

impl Default for ProfilesViewState {
    fn default() -> Self {
        Self {
            new_profile_name: String::new(),
        }
    }
}

pub fn show(ui: &mut Ui, state: &mut AppState, view_state: &mut ProfilesViewState) {
    ui.vertical(|ui| {
        ui.heading(
            RichText::new("PROFILE MANAGEMENT")
                .size(16.0)
                .color(Theme::TEXT_PRIMARY)
                .monospace(),
        );
        ui.label(
            RichText::new("Software profiles store custom key remappings with atomic disk persistence.")
                .color(Theme::TEXT_MUTED)
                .small(),
        );
        ui.add_space(10.0);

        // New Profile Creator
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("NEW PROFILE:").monospace().small().color(Theme::TEXT_MUTED));
                ui.text_edit_singleline(&mut view_state.new_profile_name);
                if ui
                    .button(RichText::new("+ Create Profile").color(Theme::ACCENT_BLUE).strong())
                    .clicked()
                    && !view_state.new_profile_name.trim().is_empty()
                {
                    let name = view_state.new_profile_name.trim().to_string();
                    state.create_profile(&name);
                    view_state.new_profile_name.clear();
                }
            });
        });

        ui.add_space(10.0);

        // Profile List Table
        let profile_ids: Vec<String> = state.profiles.keys().cloned().collect();

        egui::ScrollArea::vertical()
            .id_salt("profiles_list_scroll")
            .show(ui, |ui| {
                for id in profile_ids {
                    if let Some(prof) = state.profiles.get(&id).cloned() {
                        let is_active = state.active_profile_id == id;

                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.horizontal(|ui| {
                                // Status Indicator
                                if is_active {
                                    ui.label(
                                        RichText::new("● ACTIVE")
                                            .color(Theme::ACCENT_GREEN)
                                            .strong()
                                            .monospace(),
                                    );
                                } else {
                                    ui.label(
                                        RichText::new("○ INACTIVE")
                                            .color(Theme::TEXT_MUTED)
                                            .monospace(),
                                    );
                                }

                                ui.add_space(8.0);

                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(&prof.name)
                                            .strong()
                                            .monospace()
                                            .size(14.0)
                                            .color(if is_active {
                                                Theme::ACCENT_BLUE
                                            } else {
                                                Theme::TEXT_PRIMARY
                                            }),
                                    );
                                    ui.label(
                                        RichText::new(&prof.description)
                                            .color(Theme::TEXT_SECONDARY)
                                            .small(),
                                    );
                                    ui.label(
                                        RichText::new(format!("Mappings: {} key(s)", prof.mappings.len()))
                                            .color(Theme::TEXT_MUTED)
                                            .small()
                                            .monospace(),
                                    );
                                });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if !is_active {
                                        if ui
                                            .button(
                                                RichText::new("Activate")
                                                    .color(Theme::ACCENT_BLUE)
                                                    .strong(),
                                            )
                                            .clicked()
                                        {
                                            state.set_active_profile(&id);
                                        }
                                    }

                                    if !prof.is_readonly {
                                        if ui
                                            .button(RichText::new("✕ Delete").color(Theme::ACCENT_RED))
                                            .clicked()
                                        {
                                            state.delete_profile(&id);
                                        }
                                    } else {
                                        ui.label(
                                            RichText::new("[Read-Only Default]")
                                                .color(Theme::TEXT_MUTED)
                                                .small()
                                                .monospace(),
                                        );
                                    }
                                });
                            });
                        });
                        ui.add_space(4.0);
                    }
                }
            });
    });
}
