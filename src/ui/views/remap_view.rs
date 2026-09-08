use crate::app::state::{AppState, CaptureState};
use crate::app::theme::Theme;
use crate::input::keys::VKey;
use crate::remap::rules::MappingTarget;
use egui::{Color32, CornerRadius, RichText, Stroke, Ui};

pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        // View Title & Save/Apply Toolbar
        ui.horizontal(|ui| {
            ui.heading(
                RichText::new("KEY REMAPPING MATRIX")
                    .size(16.0)
                    .color(Theme::TEXT_PRIMARY)
                    .monospace(),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Prominent Save & Apply to System Button
                let save_btn = ui.button(
                    RichText::new("💾 SAVE & APPLY TO SYSTEM")
                        .color(Color32::WHITE)
                        .strong()
                        .monospace(),
                );
                if save_btn.clicked() {
                    state.save_and_apply_to_system();
                }

                let active_name = state
                    .profiles
                    .get(&state.active_profile_id)
                    .map(|p| p.name.as_str())
                    .unwrap_or("Default");
                ui.label(
                    RichText::new(format!("PROFILE: {}", active_name.to_uppercase()))
                        .color(Theme::ACCENT_VIOLET)
                        .monospace()
                        .small(),
                );
            });
        });

        ui.label(
            RichText::new("Hardware-level interception via low-level Windows hook. Intercepts physical keypresses and synthesizes target keys via native SendInput.")
                .color(Theme::TEXT_MUTED)
                .small(),
        );
        ui.add_space(8.0);

        // ==========================================
        // 🧪 LIVE TEST PAD (Immediate Real-time Verification)
        // ==========================================
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("🧪 LIVE TEST PAD:")
                        .monospace()
                        .strong()
                        .color(Theme::ACCENT_VIOLET),
                );
                let edit_response = ui.add(
                    egui::TextEdit::singleline(&mut state.test_input_text)
                        .hint_text("Click here and type to test remapped keys live (e.g. verify '\\' acts as Backspace)...")
                        .desired_width(ui.available_width() - 80.0),
                );
                if edit_response.has_focus() {
                    // While user is typing in test pad, ensure continuous redraw
                    ui.ctx().request_repaint();
                }
                if ui.button("Clear").clicked() {
                    state.test_input_text.clear();
                }
            });
        });

        ui.add_space(8.0);

        let target_key = state.selected_key.unwrap_or(VKey::CapsLock);

        // ==========================================
        // 1. TARGET KEY CARD
        // ==========================================
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("TARGET SOURCE KEY:")
                            .color(Theme::TEXT_MUTED)
                            .small()
                            .monospace(),
                    );
                    ui.label(
                        RichText::new(format!("{:?}", target_key))
                            .color(Theme::ACCENT_BLUE)
                            .strong()
                            .monospace()
                            .size(15.0),
                    );
                    ui.label(
                        RichText::new(format!("(VK 0x{:02X})", target_key.to_vk()))
                            .color(Theme::TEXT_MUTED)
                            .monospace()
                            .small(),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        match state.capture_state {
                            CaptureState::ListeningForTarget => {
                                if ui.button("✕ Cancel").clicked() {
                                    state.cancel_capture();
                                }
                            }
                            _ => {
                                if ui.button("🎯 Capture Physical Target").clicked() {
                                    state.start_listening_target();
                                }
                            }
                        }
                    });
                });

                // Target Capture In-Progress or Result
                match state.capture_state {
                    CaptureState::ListeningForTarget => {
                        ui.add_space(6.0);
                        render_listening_box(ui, "PRESS ANY PHYSICAL KEY TO SET AS TARGET...");
                    }
                    CaptureState::CapturedTarget(info) => {
                        ui.add_space(6.0);
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("✓ CAPTURED TARGET:")
                                        .color(Theme::ACCENT_GREEN)
                                        .strong()
                                        .monospace(),
                                );
                                ui.label(
                                    RichText::new(format!(
                                        "{:?} [Scan: 0x{:02X}, VK: 0x{:02X}]",
                                        info.vkey, info.scan_code, info.raw_vk
                                    ))
                                    .color(Theme::TEXT_PRIMARY)
                                    .monospace(),
                                );

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("✓ Confirm Target").clicked() {
                                        state.confirm_captured_target();
                                    }
                                    if ui.button("↺ Re-capture").clicked() {
                                        state.start_listening_target();
                                    }
                                });
                            });
                        });
                    }
                    _ => {}
                }
            });
        });

        ui.add_space(8.0);

        // ==========================================
        // 2. REPLACEMENT ACTION CARD
        // ==========================================
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("CURRENT ASSIGNMENT:")
                            .color(Theme::TEXT_MUTED)
                            .small()
                            .monospace(),
                    );

                    let active_profile = state.profiles.get(&state.active_profile_id);
                    let current_target = active_profile.and_then(|p| p.mappings.get(&target_key));

                    match current_target {
                        Some(MappingTarget::Key(dest)) => {
                            ui.label(
                                RichText::new(format!("REMAPPED → {:?}", dest))
                                    .color(Theme::ACCENT_BLUE)
                                    .strong()
                                    .monospace(),
                            );
                            if ui.button("↺ Reset to 1:1").clicked() {
                                state.remove_mapping(target_key);
                            }
                        }
                        Some(MappingTarget::Block) => {
                            ui.label(
                                RichText::new("[KEY BLOCKED]")
                                    .color(Theme::ACCENT_RED)
                                    .strong()
                                    .monospace(),
                            );
                            if ui.button("↺ Unblock").clicked() {
                                state.remove_mapping(target_key);
                            }
                        }
                        None => {
                            ui.label(
                                RichText::new("1:1 Pass-Through (Default)")
                                    .color(Theme::TEXT_SECONDARY)
                                    .monospace(),
                            );
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("✕ Block Key").clicked() {
                            state.set_mapping(target_key, MappingTarget::Block);
                        }
                    });
                });

                ui.add_space(8.0);

                // Main Capture Replacement Action
                match state.capture_state {
                    CaptureState::ListeningForReplacement => {
                        render_listening_box(ui, "PRESS ANY PHYSICAL KEY FOR REPLACEMENT...");
                        ui.add_space(4.0);
                        if ui.button("✕ Cancel Capture").clicked() {
                            state.cancel_capture();
                        }
                    }
                    CaptureState::CapturedReplacement(info) => {
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new("✓ CAPTURED REPLACEMENT KEY:")
                                            .color(Theme::ACCENT_GREEN)
                                            .strong()
                                            .monospace(),
                                    );
                                    ui.label(
                                        RichText::new(format!("{:?}", info.vkey))
                                            .color(Theme::ACCENT_BLUE)
                                            .strong()
                                            .monospace()
                                            .size(15.0),
                                    );
                                    ui.label(
                                        RichText::new(format!(
                                            "[Scan: 0x{:04X} | VK: 0x{:04X}]",
                                            info.scan_code, info.raw_vk
                                        ))
                                        .color(Theme::TEXT_MUTED)
                                        .monospace()
                                        .small(),
                                    );
                                });

                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    let apply_btn = ui.button(
                                        RichText::new(format!(
                                            "✓ APPLY REMAP: {:?} → {:?}",
                                            target_key, info.vkey
                                        ))
                                        .color(Theme::ACCENT_BLUE)
                                        .strong(),
                                    );
                                    if apply_btn.clicked() {
                                        state.confirm_captured_replacement();
                                    }

                                    // Interchanging / Key Swapping Feature!
                                    let swap_btn = ui.button(
                                        RichText::new(format!(
                                            "⇄ SWAP KEYS ({:?} ↔ {:?})",
                                            target_key, info.vkey
                                        ))
                                        .color(Theme::ACCENT_VIOLET)
                                        .strong(),
                                    );
                                    if swap_btn.clicked() {
                                        state.swap_mappings(target_key, info.vkey);
                                        state.cancel_capture();
                                    }

                                    if ui.button("↺ Re-capture").clicked() {
                                        state.start_listening_replacement();
                                    }

                                    if ui.button("Cancel").clicked() {
                                        state.cancel_capture();
                                    }
                                });
                            });
                        });
                    }
                    _ => {
                        ui.horizontal(|ui| {
                            let capture_btn = ui.button(
                                RichText::new("⚡ Capture Physical Replacement Key")
                                    .color(Theme::ACCENT_BLUE)
                                    .strong(),
                            );
                            if capture_btn.clicked() {
                                state.start_listening_replacement();
                            }
                            ui.label(
                                RichText::new("(Press any key on hardware to bind)")
                                    .color(Theme::TEXT_MUTED)
                                    .small(),
                            );
                        });
                    }
                }
            });
        });

        ui.add_space(8.0);

        // ==========================================
        // 3. MANUAL FALLBACK KEY PICKER (Collapsed)
        // ==========================================
        let toggle_label = if state.show_manual_picker {
            "▼ Hide Manual Key Picker"
        } else {
            "▶ Or choose replacement manually (accessibility fallback)"
        };

        if ui.button(RichText::new(toggle_label).small().color(Theme::TEXT_MUTED)).clicked() {
            state.show_manual_picker = !state.show_manual_picker;
        }

        if state.show_manual_picker {
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                egui::ScrollArea::vertical()
                    .id_salt("remap_replacement_key_grid")
                    .max_height(160.0)
                    .show(ui, |ui| {
                        ui.label(RichText::new("Common Controls:").small().color(Theme::TEXT_MUTED));
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
                                    state.set_mapping(target_key, MappingTarget::Key(target));
                                }
                            }
                        });

                        ui.add_space(6.0);
                        ui.label(RichText::new("Alphanumeric Keys:").small().color(Theme::TEXT_MUTED));
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
                                    state.set_mapping(target_key, MappingTarget::Key(k));
                                }
                            }
                        });
                    });
            });
        }

        ui.add_space(10.0);

        // ==========================================
        // 4. ACTIVE REMAPPED KEYS TABLE
        // ==========================================
        ui.horizontal(|ui| {
            ui.heading(
                RichText::new("ACTIVE REMAPPINGS IN CURRENT PROFILE")
                    .size(14.0)
                    .monospace()
                    .color(Theme::TEXT_PRIMARY),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("💾 Apply to System").clicked() {
                    state.save_and_apply_to_system();
                }
            });
        });
        ui.add_space(4.0);

        let mappings_clone = {
            let prof = state.profiles.get(&state.active_profile_id);
            prof.map(|p| p.mappings.clone()).unwrap_or_default()
        };

        if mappings_clone.is_empty() {
            ui.label(
                RichText::new("No custom mappings in this profile. All physical keys pass through 1:1.")
                    .color(Theme::TEXT_MUTED)
                    .small(),
            );
        } else {
            egui::ScrollArea::vertical()
                .id_salt("remap_active_mappings_list")
                .max_height(180.0)
                .show(ui, |ui| {
                    for (src, target) in mappings_clone {
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format!("{:?}", src))
                                        .strong()
                                        .monospace(),
                                );
                                ui.label(RichText::new("→").color(Theme::TEXT_MUTED));
                                match target {
                                    MappingTarget::Key(dest) => {
                                        ui.label(
                                            RichText::new(format!("{:?}", dest))
                                                .color(Theme::ACCENT_BLUE)
                                                .strong()
                                                .monospace(),
                                        );
                                    }
                                    MappingTarget::Block => {
                                        ui.label(
                                            RichText::new("[BLOCKED]")
                                                .color(Theme::ACCENT_RED)
                                                .strong()
                                                .monospace(),
                                        );
                                    }
                                }

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("✕ Remove").clicked() {
                                        state.remove_mapping(src);
                                    }
                                });
                            });
                        });
                        ui.add_space(2.0);
                    }
                });
        }
    });
}

/// Renders a pulsing retro terminal capture box
fn render_listening_box(ui: &mut Ui, message: &str) {
    let time = ui.input(|i| i.time);
    let pulse = ((time * 4.0).sin() * 0.5 + 0.5) as f32;
    let border_color = Color32::from_rgb(
        (163.0 * (0.6 + 0.4 * pulse)) as u8,
        (116.0 * (0.6 + 0.4 * pulse)) as u8,
        (255.0 * (0.6 + 0.4 * pulse)) as u8,
    );

    let (rect, _response) = ui.allocate_exact_size(
        egui::Vec2::new(ui.available_width(), 44.0),
        egui::Sense::hover(),
    );
    let painter = ui.painter_at(rect);

    painter.rect_filled(
        rect,
        CornerRadius::same(1),
        Color32::from_rgba_premultiplied(0x18, 0x14, 0x28, 200),
    );
    painter.rect_stroke(
        rect,
        CornerRadius::same(1),
        Stroke::new(1.5_f32, border_color),
        egui::StrokeKind::Inside,
    );

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("⚡ {}", message),
        egui::FontId::monospace(13.0),
        Color32::from_rgb(
            (228.0 + 27.0 * pulse) as u8,
            (228.0 + 27.0 * pulse) as u8,
            255,
        ),
    );
}
