use crate::app::state::AppState;
use crate::app::theme::Theme;
use crate::input::injector::KCC_MAGIC_EXTRA_INFO;
use egui::{RichText, Ui};

pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("Application Settings & Configuration").size(18.0).color(Theme::TEXT_PRIMARY));
        ui.add_space(12.0);

        // System Info Card
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.label(RichText::new("Runtime Architecture").strong().size(14.0));
                ui.add_space(6.0);

                egui::Grid::new("sys_info_grid").min_col_width(180.0).show(ui, |ui| {
                    ui.label("Application Name:");
                    ui.label(RichText::new("Theasus Keyboard Control Center").strong());
                    ui.end_row();

                    ui.label("Build Target:");
                    ui.label(RichText::new("Windows x86_64 Native (Rust eframe/egui)").monospace());
                    ui.end_row();

                    ui.label("Auto-Detection Subsystem:");
                    ui.label(RichText::new("Windows Raw Input (hDevice Keystroke Resolution)").monospace().color(Theme::ACCENT_GREEN));
                    ui.end_row();

                    ui.label("Input Interception:");
                    ui.label(RichText::new("Windows WH_KEYBOARD_LL (Microsecond Worker)").monospace());
                    ui.end_row();

                    ui.label("Anti-Recursion Tag:");
                    ui.label(RichText::new(format!("0x{:08X} (\"KCC1\")", KCC_MAGIC_EXTRA_INFO)).monospace().color(Theme::ACCENT_CYAN));
                    ui.end_row();

                    ui.label("Configuration Directory:");
                    ui.label(RichText::new(state.storage.get_storage_path().to_string_lossy()).monospace().small());
                    ui.end_row();
                });
            });
        });

        ui.add_space(12.0);

        // Factory Reset Actions
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.label(RichText::new("Profile Reset Actions").strong().size(14.0));
                ui.label(RichText::new("Reverts custom profiles and mappings to baseline configurations.")
                    .color(Theme::TEXT_SECONDARY).small());

                ui.add_space(8.0);

                if ui.button("↺ Reinitialize Default Profiles").clicked() {
                    let default_prof = crate::profiles::model::Profile::new_default();
                    let gaming_prof = crate::profiles::model::Profile::new_gaming();
                    let work_prof = crate::profiles::model::Profile::new_work();

                    let _ = state.storage.save_profile(&default_prof);
                    let _ = state.storage.save_profile(&gaming_prof);
                    let _ = state.storage.save_profile(&work_prof);

                    state.profiles.insert(default_prof.id.clone(), default_prof);
                    state.profiles.insert(gaming_prof.id.clone(), gaming_prof);
                    state.profiles.insert(work_prof.id.clone(), work_prof);
                    state.set_active_profile("default");
                    state.set_status("Reinitialized standard profiles (Default, Gaming, Work)");
                }
            });
        });
    });
}
