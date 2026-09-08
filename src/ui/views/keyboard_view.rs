use crate::app::state::{AppState, CaptureState};
use crate::app::theme::Theme;
use crate::input::keys::VKey;
use crate::keyboard_visual::KeyboardRenderer;
use crate::remap::rules::MappingTarget;
use egui::{Color32, RichText, Ui};

pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        // Workspace Header
        ui.horizontal(|ui| {
            ui.heading(
                RichText::new("Visual Keyboard")
                    .size(18.0)
                    .color(Theme::TEXT_PRIMARY)
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Save & Apply to System Button
                let save_btn = ui.button(
                    RichText::new("💾 Save & Apply to System")
                        .color(Color32::WHITE)
                        .strong()
                        .size(12.5),
                );
                if save_btn.clicked() {
                    state.save_and_apply_to_system();
                }

                let profile_name = state
                    .profiles
                    .get(&state.active_profile_id)
                    .map(|p| p.name.as_str())
                    .unwrap_or("Default");
                ui.label(
                    RichText::new(format!("PROFILE: {}", profile_name.to_uppercase()))
                        .color(Theme::ACCENT_VIOLET)
                        .strong()
                        .size(11.0),
                );
            });
        });
        ui.add_space(8.0);

        // Visual Keyboard Area (CBG K26 68-Key ANSI Mechanical)
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.add_space(6.0);
                let active_profile = state
                    .profiles
                    .get(&state.active_profile_id)
                    .cloned()
                    .unwrap_or_else(crate::profiles::model::Profile::new_default);

                let renderer = KeyboardRenderer::new(
                    &state.pressed_keys,
                    state.selected_key,
                    &active_profile,
                );

                if let Some(clicked) = renderer.render(ui) {
                    state.selected_key = Some(clicked);
                    state.cancel_capture();
                }
                ui.add_space(6.0);
            });
        });

        ui.add_space(8.0);

        // ==========================================
        // 🧪 LIVE TEST PAD (Immediate Real-time Verification)
        // ==========================================
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("🧪 LIVE TEST PAD:")
                        .strong()
                        .color(Theme::ACCENT_CYAN),
                );
                let edit_response = ui.add(
                    egui::TextEdit::singleline(&mut state.test_input_text)
                        .hint_text("Type here to test your remapped keys live across the system...")
                        .desired_width(ui.available_width() - 80.0),
                );
                if edit_response.has_focus() {
                    ui.ctx().request_repaint();
                }
                if ui.button("Clear").clicked() {
                    state.test_input_text.clear();
                }
            });
        });

        ui.add_space(8.0);

        // Selected Key Quick-Remap Card
        if let Some(selected_vkey) = state.selected_key {
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("SELECTED KEY:")
                                .color(Theme::TEXT_MUTED)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new(format!("{:?}", selected_vkey))
                                .strong()
                                .color(Theme::ACCENT_BLUE)
                                .size(14.0),
                        );
                        ui.label(
                            RichText::new(format!("(VK 0x{:02X})", selected_vkey.to_vk()))
                                .color(Theme::TEXT_MUTED)
                                .monospace()
                                .small(),
                        );

                        let active_profile = state.profiles.get(&state.active_profile_id);
                        let current_target = active_profile.and_then(|p| p.mappings.get(&selected_vkey));

                        ui.separator();

                        match current_target {
                            Some(MappingTarget::Key(dest)) => {
                                ui.label(
                                    RichText::new(format!("Remapped → {:?}", dest))
                                        .color(Theme::ACCENT_BLUE)
                                        .strong(),
                                );
                                if ui.button("↺ Reset").clicked() {
                                    state.remove_mapping(selected_vkey);
                                }
                            }
                            Some(MappingTarget::Block) => {
                                ui.label(
                                    RichText::new("[BLOCKED]")
                                        .color(Theme::ACCENT_RED)
                                        .strong(),
                                );
                                if ui.button("↺ Unblock").clicked() {
                                    state.remove_mapping(selected_vkey);
                                }
                            }
                            None => {
                                ui.label(
                                    RichText::new("Assignment: 1:1 Pass-Through")
                                        .color(Theme::TEXT_SECONDARY)
                                        .small(),
                                );
                            }
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            match state.capture_state {
                                CaptureState::ListeningForTarget => {
                                    if ui.button("✕ Cancel").clicked() {
                                        state.cancel_capture();
                                    }
                                }
                                _ => {
                                    if ui.button("🎯 Capture Target Key").clicked() {
                                        state.start_listening_target();
                                    }
                                }
                            }

                            if ui.button("✕ Block Key").clicked() {
                                state.set_mapping(selected_vkey, MappingTarget::Block);
                            }
                        });
                    });

                    ui.add_space(6.0);

                    // Quick Assign Desired Key (1-Click Remapping)
                    ui.label(
                        RichText::new("QUICK ASSIGN DESIRED KEY:")
                            .color(Theme::TEXT_MUTED)
                            .size(11.0),
                    );
                    ui.horizontal_wrapped(|ui| {
                        let quick_keys = [
                            (VKey::Backspace, "⌫ Backspace"),
                            (VKey::Enter, "↵ Enter"),
                            (VKey::Escape, "⎋ Esc"),
                            (VKey::Delete, "⌦ Del"),
                            (VKey::Tab, "⇥ Tab"),
                            (VKey::Space, "Space"),
                            (VKey::CapsLock, "Caps"),
                            (VKey::ControlLeft, "Ctrl (L)"),
                            (VKey::AltLeft, "Alt (L)"),
                            (VKey::ShiftLeft, "Shift (L)"),
                            (VKey::WinLeft, "⊞ Win"),
                            (VKey::ArrowUp, "▲ Up"),
                            (VKey::ArrowDown, "▼ Down"),
                            (VKey::ArrowLeft, "◀ Left"),
                            (VKey::ArrowRight, "▶ Right"),
                        ];

                        let current_mapping = state
                            .profiles
                            .get(&state.active_profile_id)
                            .and_then(|p| p.mappings.get(&selected_vkey))
                            .cloned();

                        for (vkey, label) in quick_keys {
                            let is_current = match &current_mapping {
                                Some(MappingTarget::Key(dest)) => *dest == vkey,
                                _ => false,
                            };

                            let btn = if is_current {
                                ui.button(
                                    RichText::new(label)
                                        .color(Theme::ACCENT_BLUE)
                                        .strong(),
                                )
                            } else {
                                ui.button(RichText::new(label))
                            };

                            if btn.clicked() {
                                state.set_mapping(selected_vkey, MappingTarget::Key(vkey));
                            }
                        }
                    });

                    ui.add_space(6.0);

                    // Smart Contextual Interchange / Swap Keys
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("INTERCHANGE / SWAP:")
                                .color(Theme::TEXT_MUTED)
                                .size(11.0),
                        );

                        if selected_vkey == VKey::Backslash {
                            if ui
                                .button(
                                    RichText::new("⇄ Swap: \\ ↔ Backspace")
                                        .color(Theme::ACCENT_VIOLET)
                                        .strong(),
                                )
                                .clicked()
                            {
                                state.swap_mappings(VKey::Backslash, VKey::Backspace);
                            }
                        } else if selected_vkey == VKey::Backspace {
                            if ui
                                .button(
                                    RichText::new("⇄ Swap: Backspace ↔ \\")
                                        .color(Theme::ACCENT_VIOLET)
                                        .strong(),
                                )
                                .clicked()
                            {
                                state.swap_mappings(VKey::Backspace, VKey::Backslash);
                            }
                        } else if selected_vkey == VKey::CapsLock {
                            if ui
                                .button(
                                    RichText::new("⇄ Swap: CapsLock ↔ Ctrl")
                                        .color(Theme::ACCENT_VIOLET)
                                        .strong(),
                                )
                                .clicked()
                            {
                                state.swap_mappings(VKey::CapsLock, VKey::ControlLeft);
                            }
                            if ui
                                .button(
                                    RichText::new("⇄ Swap: CapsLock ↔ Esc")
                                        .color(Theme::ACCENT_VIOLET)
                                        .strong(),
                                )
                                .clicked()
                            {
                                state.swap_mappings(VKey::CapsLock, VKey::Escape);
                            }
                        }

                        // Arbitrary Swap Target
                        let swap_target = state.swap_target_key.unwrap_or(VKey::Backspace);
                        egui::ComboBox::from_id_salt("kb_swap_target_combo")
                            .selected_text(swap_target.name())
                            .width(130.0)
                            .show_ui(ui, |ui| {
                                for &k in VKey::all_assignable() {
                                    if k != selected_vkey {
                                        ui.selectable_value(&mut state.swap_target_key, Some(k), k.name());
                                    }
                                }
                            });

                        if ui
                            .button(
                                RichText::new(format!("⇄ Swap with {:?}", swap_target))
                                    .color(Theme::ACCENT_VIOLET),
                            )
                            .clicked()
                        {
                            state.swap_mappings(selected_vkey, swap_target);
                        }
                    });

                    ui.add_space(6.0);

                    // Capture State Feedback Area
                    match state.capture_state {
                        CaptureState::ListeningForTarget => {
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new("⚡ PRESS ANY PHYSICAL KEY TO SELECT TARGET...")
                                    .color(Theme::ACCENT_VIOLET)
                                    .strong()
                                    .monospace(),
                            );
                        }
                        CaptureState::ListeningForReplacement => {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("⚡ PRESS ANY PHYSICAL KEY TO BIND AS REPLACEMENT...")
                                        .color(Theme::ACCENT_VIOLET)
                                        .strong()
                                        .monospace(),
                                );
                                if ui.button("Cancel").clicked() {
                                    state.cancel_capture();
                                }
                            });
                        }
                        CaptureState::CapturedReplacement(info) => {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "✓ CAPTURED: {:?} (Scan 0x{:02X})",
                                        info.vkey, info.scan_code
                                    ))
                                    .color(Theme::ACCENT_GREEN)
                                    .strong()
                                    .monospace(),
                                );
                                if ui
                                    .button(
                                        RichText::new(format!(
                                            "✓ Apply Remap: {:?} → {:?}",
                                            selected_vkey, info.vkey
                                        ))
                                        .color(Theme::ACCENT_BLUE)
                                        .strong(),
                                    )
                                    .clicked()
                                {
                                    state.confirm_captured_replacement();
                                }
                                if ui
                                    .button(
                                        RichText::new(format!(
                                            "⇄ Swap: {:?} ↔ {:?}",
                                            selected_vkey, info.vkey
                                        ))
                                        .color(Theme::ACCENT_VIOLET)
                                        .strong(),
                                    )
                                    .clicked()
                                {
                                    state.swap_mappings(selected_vkey, info.vkey);
                                    state.cancel_capture();
                                }
                                if ui.button("↺ Re-capture").clicked() {
                                    state.start_listening_replacement();
                                }
                                if ui.button("Cancel").clicked() {
                                    state.cancel_capture();
                                }
                            });
                        }
                        _ => {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                if ui
                                    .button(
                                        RichText::new("⚡ Capture Physical Replacement Key")
                                            .color(Theme::ACCENT_BLUE)
                                            .strong(),
                                    )
                                    .clicked()
                                {
                                    state.start_listening_replacement();
                                }
                                ui.label(
                                    RichText::new("Press physical key or use 1-click presets above.")
                                        .color(Theme::TEXT_MUTED)
                                        .small(),
                                );
                            });
                        }
                    }
                });
            });
        }

        ui.add_space(8.0);

        // Hardware Attribution Status
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("IDENTIFY-BY-KEYSTROKE:")
                    .color(Theme::ACCENT_GREEN)
                    .small()
                    .monospace()
                    .strong(),
            );
            ui.label(
                RichText::new("Physical keystrokes are intercepted live with microsecond resolution and zero polling.")
                    .color(Theme::TEXT_MUTED)
                    .small(),
            );
        });
    });
}
