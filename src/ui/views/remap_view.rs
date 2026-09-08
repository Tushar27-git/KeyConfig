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
            RichText::new("Hardware-level interception via low-level Windows hook (WH_KEYBOARD_LL). Intercepts physical keypresses and synthesizes target keys via native SendInput.")
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
                        .hint_text("Click here and type to test remapped keys live across Windows (e.g. verify '\\' deletes as Backspace)...")
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

        let target_key = state.selected_key.unwrap_or(VKey::Backslash);

        // ==========================================
        // 1. TARGET SOURCE KEY CARD
        // ==========================================
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("TARGET PHYSICAL KEY:")
                            .color(Theme::TEXT_MUTED)
                            .small()
                            .monospace(),
                    );
                    ui.label(
                        RichText::new(format!("{:?} ({})", target_key, target_key.label()))
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

                    let active_profile = state.profiles.get(&state.active_profile_id);
                    let current_target = active_profile.and_then(|p| p.mappings.get(&target_key));

                    ui.separator();
                    match current_target {
                        Some(MappingTarget::Key(dest)) => {
                            ui.label(
                                RichText::new(format!("REMAPPED → {:?}", dest))
                                    .color(Theme::ACCENT_BLUE)
                                    .strong()
                                    .monospace(),
                            );
                            if ui.button("↺ Reset").clicked() {
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
                                    .monospace()
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
                                if ui.button("🎯 Capture Physical Target").clicked() {
                                    state.start_listening_target();
                                }
                            }
                        }

                        if ui.button("✕ Block Key").clicked() {
                            state.set_mapping(target_key, MappingTarget::Block);
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

                ui.add_space(6.0);

                // ==========================================
                // 1-CLICK QUICK ASSIGN PRESETS
                // ==========================================
                ui.label(
                    RichText::new("ASSIGN DESIRED OUTPUT KEY (1-Click Presets):")
                        .color(Theme::TEXT_MUTED)
                        .small()
                        .monospace(),
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
                        (VKey::VolumeMute, "🔇 Mute"),
                        (VKey::VolumeDown, "🔉 Vol-"),
                        (VKey::VolumeUp, "🔊 Vol+"),
                    ];

                    let current_target = state
                        .profiles
                        .get(&state.active_profile_id)
                        .and_then(|p| p.mappings.get(&target_key))
                        .cloned();

                    for (vkey, label) in quick_keys {
                        let is_current = match &current_target {
                            Some(MappingTarget::Key(dest)) => *dest == vkey,
                            _ => false,
                        };

                        let btn = if is_current {
                            ui.button(
                                RichText::new(label)
                                    .color(Theme::ACCENT_BLUE)
                                    .strong()
                                    .monospace(),
                            )
                        } else {
                            ui.button(RichText::new(label).monospace())
                        };

                        if btn.clicked() {
                            state.set_mapping(target_key, MappingTarget::Key(vkey));
                        }
                    }
                });

                ui.add_space(6.0);

                // ==========================================
                // KEY INTERCHANGE / SWAPPING SECTION
                // ==========================================
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("INTERCHANGE / SWAP:")
                            .color(Theme::TEXT_MUTED)
                            .small()
                            .monospace(),
                    );

                    if target_key == VKey::Backslash {
                        if ui
                            .button(
                                RichText::new("⇄ SWAP: \\ ↔ Backspace")
                                    .color(Theme::ACCENT_VIOLET)
                                    .strong()
                                    .monospace(),
                            )
                            .clicked()
                        {
                            state.swap_mappings(VKey::Backslash, VKey::Backspace);
                        }
                    } else if target_key == VKey::Backspace {
                        if ui
                            .button(
                                RichText::new("⇄ SWAP: Backspace ↔ \\")
                                    .color(Theme::ACCENT_VIOLET)
                                    .strong()
                                    .monospace(),
                            )
                            .clicked()
                        {
                            state.swap_mappings(VKey::Backspace, VKey::Backslash);
                        }
                    } else if target_key == VKey::CapsLock {
                        if ui
                            .button(
                                RichText::new("⇄ SWAP: CapsLock ↔ Ctrl")
                                    .color(Theme::ACCENT_VIOLET)
                                    .strong()
                                    .monospace(),
                            )
                            .clicked()
                        {
                            state.swap_mappings(VKey::CapsLock, VKey::ControlLeft);
                        }
                        if ui
                            .button(
                                RichText::new("⇄ SWAP: CapsLock ↔ Esc")
                                    .color(Theme::ACCENT_VIOLET)
                                    .strong()
                                    .monospace(),
                            )
                            .clicked()
                        {
                            state.swap_mappings(VKey::CapsLock, VKey::Escape);
                        }
                    }

                    // Swap With Any Key Selector
                    let swap_target = state.swap_target_key.unwrap_or(VKey::Backspace);
                    egui::ComboBox::from_id_salt("remap_swap_combo")
                        .selected_text(swap_target.name())
                        .width(130.0)
                        .show_ui(ui, |ui| {
                            for &k in VKey::all_assignable() {
                                if k != target_key {
                                    ui.selectable_value(&mut state.swap_target_key, Some(k), k.name());
                                }
                            }
                        });

                    if ui
                        .button(
                            RichText::new(format!("⇄ Swap with {:?}", swap_target))
                                .color(Theme::ACCENT_VIOLET)
                                .monospace(),
                        )
                        .clicked()
                    {
                        state.swap_mappings(target_key, swap_target);
                    }
                });

                ui.add_space(6.0);

                // Physical Capture Replacement Flow
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
                                RichText::new("(Press any hardware key to bind live)")
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
        // 2. ADD NEW REMAPPING RULE BUILDER
        // ==========================================
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("ADD NEW REMAPPING RULE TO MATRIX:")
                        .color(Theme::TEXT_PRIMARY)
                        .strong()
                        .monospace()
                        .size(13.0),
                );
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("When I press:").color(Theme::TEXT_MUTED).small());

                    let src = state.matrix_new_source.unwrap_or(VKey::Backslash);
                    egui::ComboBox::from_id_salt("matrix_src_combo")
                        .selected_text(src.name())
                        .width(140.0)
                        .show_ui(ui, |ui| {
                            for &k in VKey::all_assignable() {
                                ui.selectable_value(&mut state.matrix_new_source, Some(k), k.name());
                            }
                        });

                    ui.label(RichText::new("➔  Produce:").color(Theme::TEXT_MUTED).strong());

                    let dest = state.matrix_new_target.unwrap_or(VKey::Backspace);
                    egui::ComboBox::from_id_salt("matrix_dest_combo")
                        .selected_text(dest.name())
                        .width(140.0)
                        .show_ui(ui, |ui| {
                            for &k in VKey::all_assignable() {
                                ui.selectable_value(&mut state.matrix_new_target, Some(k), k.name());
                            }
                        });

                    if ui
                        .button(
                            RichText::new("➕ Add / Update Remap")
                                .color(Theme::ACCENT_BLUE)
                                .strong(),
                        )
                        .clicked()
                    {
                        state.set_mapping(src, MappingTarget::Key(dest));
                    }

                    if ui
                        .button(
                            RichText::new(format!("⇄ Swap Both ({:?} ↔ {:?})", src, dest))
                                .color(Theme::ACCENT_VIOLET)
                                .strong(),
                        )
                        .clicked()
                    {
                        state.swap_mappings(src, dest);
                    }
                });
            });
        });

        ui.add_space(8.0);

        // ==========================================
        // 3. ACTIVE REMAPPED KEYS TABLE (Matrix)
        // ==========================================
        let mappings_clone = {
            let prof = state.profiles.get(&state.active_profile_id);
            prof.map(|p| p.mappings.clone()).unwrap_or_default()
        };

        ui.horizontal(|ui| {
            ui.heading(
                RichText::new(format!(
                    "ACTIVE REMAPPINGS IN CURRENT PROFILE ({})",
                    mappings_clone.len()
                ))
                .size(14.0)
                .monospace()
                .color(Theme::TEXT_PRIMARY),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("💾 Apply to System").clicked() {
                    state.save_and_apply_to_system();
                }

                if !mappings_clone.is_empty() {
                    if ui.button(RichText::new("🗑 Clear All").color(Theme::ACCENT_RED)).clicked() {
                        state.clear_all_mappings();
                    }
                }
            });
        });
        ui.add_space(4.0);

        if mappings_clone.is_empty() {
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.label(
                    RichText::new("No custom mappings in this profile. All physical keys pass through 1:1.\nUse the controls above to assign Backspace, swap keys, or capture any keystroke.")
                        .color(Theme::TEXT_MUTED)
                        .small(),
                );
            });
        } else {
            egui::ScrollArea::vertical()
                .id_salt("remap_active_mappings_list")
                .max_height(200.0)
                .show(ui, |ui| {
                    for (src, target) in mappings_clone {
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format!("{:?} ({})", src, src.label()))
                                        .strong()
                                        .monospace(),
                                );
                                ui.label(RichText::new("→").color(Theme::TEXT_MUTED).strong());

                                match target {
                                    MappingTarget::Key(dest) => {
                                        ui.label(
                                            RichText::new(format!("{:?} ({})", dest, dest.label()))
                                                .color(Theme::ACCENT_BLUE)
                                                .strong()
                                                .monospace(),
                                        );

                                        // 1-Click Swap Button for each active mapping
                                        if ui
                                            .button(
                                                RichText::new(format!("⇄ Swap {:?} ↔ {:?}", src, dest))
                                                    .color(Theme::ACCENT_VIOLET)
                                                    .small(),
                                            )
                                            .clicked()
                                        {
                                            state.swap_mappings(src, dest);
                                        }
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
