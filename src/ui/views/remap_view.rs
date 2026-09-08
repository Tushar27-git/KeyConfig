use crate::app::state::AppState;
use crate::app::theme::Theme;
use crate::input::keys::VKey;
use crate::remap::rules::MappingTarget;
use egui::{RichText, Ui};

pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("Key Remapping Matrix").size(18.0).color(Theme::TEXT_PRIMARY));
        ui.label(RichText::new("Configure deterministic software key remapping. Injected events are strictly tagged to prevent loops.").color(Theme::TEXT_SECONDARY));
        ui.add_space(12.0);

        let selected = state.selected_key.unwrap_or(VKey::CapsLock);

        // Remap Editor Toolbar
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Target Source: {:?}", selected)).strong().size(15.0));

                if state.capture_mode {
                    ui.label(RichText::new("⚡ Press a physical key to capture...").color(Theme::ACCENT_AMBER).strong());
                    if ui.button("Cancel").clicked() {
                        state.capture_mode = false;
                    }
                } else if ui.button("🎯 Capture Physical Key").clicked() {
                    state.capture_mode = true;
                }

                if ui.button("✕ Block Key").clicked() {
                    state.set_mapping(selected, MappingTarget::Block);
                }

                if ui.button("↺ Reset to 1:1").clicked() {
                    state.remove_mapping(selected);
                }
            });
        });

        ui.add_space(12.0);

        // Target Key Palette
        ui.label(RichText::new("Select Replacement Key:").strong());
        egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let common_targets = [
                    VKey::Escape, VKey::CapsLock, VKey::ControlLeft, VKey::ControlRight,
                    VKey::ShiftLeft, VKey::ShiftRight, VKey::AltLeft, VKey::AltRight,
                    VKey::Backspace, VKey::Enter, VKey::Delete, VKey::Tab,
                    VKey::ArrowUp, VKey::ArrowDown, VKey::ArrowLeft, VKey::ArrowRight,
                    VKey::Home, VKey::End, VKey::PageUp, VKey::PageDown,
                    VKey::VolumeMute, VKey::VolumeDown, VKey::VolumeUp,
                    VKey::MediaPlayPause, VKey::MediaNext, VKey::MediaPrev,
                ];

                for target in common_targets {
                    if ui.button(format!("{:?}", target)).clicked() {
                        state.set_mapping(selected, MappingTarget::Key(target));
                    }
                }
            });

            ui.add_space(8.0);
            ui.label(RichText::new("Alphanumeric:").small().color(Theme::TEXT_MUTED));
            ui.horizontal_wrapped(|ui| {
                let alpha = [
                    VKey::KeyA, VKey::KeyB, VKey::KeyC, VKey::KeyD, VKey::KeyE, VKey::KeyF,
                    VKey::KeyG, VKey::KeyH, VKey::KeyI, VKey::KeyJ, VKey::KeyK, VKey::KeyL,
                    VKey::KeyM, VKey::KeyN, VKey::KeyO, VKey::KeyP, VKey::KeyQ, VKey::KeyR,
                    VKey::KeyS, VKey::KeyT, VKey::KeyU, VKey::KeyV, VKey::KeyW, VKey::KeyX,
                    VKey::KeyY, VKey::KeyZ,
                ];
                for k in alpha {
                    if ui.button(k.label()).clicked() {
                        state.set_mapping(selected, MappingTarget::Key(k));
                    }
                }
            });
        });

        ui.add_space(16.0);

        // Active Mappings Table
        ui.heading(RichText::new("Active Remapped Keys in Current Profile").size(15.0));

        let mappings_clone = {
            let prof = state.profiles.get(&state.active_profile_id);
            prof.map(|p| p.mappings.clone()).unwrap_or_default()
        };

        if mappings_clone.is_empty() {
            ui.label(RichText::new("No active custom mappings. All keys are standard 1:1 pass-through.").color(Theme::TEXT_MUTED));
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (src, target) in mappings_clone {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(format!("{:?}", src)).strong());
                            ui.label("→");
                            match target {
                                MappingTarget::Key(dest) => {
                                    ui.label(RichText::new(format!("{:?}", dest)).color(Theme::ACCENT_CYAN).strong());
                                }
                                MappingTarget::Block => {
                                    ui.label(RichText::new("[BLOCKED]").color(Theme::ACCENT_RED).strong());
                                }
                            }

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Remove").clicked() {
                                    state.remove_mapping(src);
                                }
                            });
                        });
                    });
                    ui.add_space(4.0);
                }
            });
        }
    });
}
