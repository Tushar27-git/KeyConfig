use crate::app::state::AppState;
use crate::app::theme::Theme;
use crate::input::injector::KCC_MAGIC_EXTRA_INFO;
use crate::startup::StartupLaunchMode;
use egui::{RichText, Ui};

pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        ui.heading(
            RichText::new("Application Settings & Configuration")
                .size(18.0)
                .color(Theme::TEXT_PRIMARY),
        );
        ui.add_space(12.0);

        // Windows Auto-Startup Card
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("⚡ Windows Auto-Startup System").strong().size(14.0));
                    ui.separator();
                    if state.startup_status.enabled {
                        ui.label(
                            RichText::new("● ACTIVE (BOOT & RESTART)")
                                .color(Theme::ACCENT_GREEN)
                                .strong()
                                .size(11.0),
                        );
                    } else {
                        ui.label(
                            RichText::new("○ DISABLED")
                                .color(Theme::ACCENT_AMBER)
                                .strong()
                                .size(11.0),
                        );
                    }
                });

                ui.add_space(4.0);
                ui.label(
                    RichText::new(
                        "Automatically launch Theasus whenever your PC boots, restarts, or user logs in. Key remappings activate immediately upon Windows session startup.",
                    )
                    .color(Theme::TEXT_SECONDARY)
                    .small(),
                );

                ui.add_space(8.0);

                // Startup Mode Radio Selection
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Startup Launch Mode:").strong().small());
                    let mut mode = state.selected_startup_mode;
                    let norm_resp =
                        ui.radio_value(&mut mode, StartupLaunchMode::Normal, "Normal Window");
                    let min_resp = ui.radio_value(
                        &mut mode,
                        StartupLaunchMode::Minimized,
                        "Start Minimized (Background Tray/Taskbar)",
                    );

                    if norm_resp.changed() || min_resp.changed() {
                        state.selected_startup_mode = mode;
                        if state.startup_status.enabled {
                            state.enable_autostart();
                        }
                    }
                });

                ui.add_space(10.0);

                // Action Buttons
                ui.horizontal(|ui| {
                    if state.startup_status.enabled {
                        if ui.button("❌ Disable Auto-Startup").clicked() {
                            state.disable_autostart();
                        }

                        if !state.startup_status.is_current_exe {
                            if ui.button("🔄 Update Executable Path").clicked() {
                                state.enable_autostart();
                            }
                        }
                    } else {
                        if ui.button("🚀 Enable Auto-Startup").clicked() {
                            state.enable_autostart();
                        }
                    }

                    if ui.button("↻ Refresh Status").clicked() {
                        state.refresh_startup_status();
                        state.set_status("Checked Windows startup registry status");
                    }
                });

                ui.add_space(8.0);

                // Collapsible Technical Registry Info
                ui.collapsing("Technical Details & Windows Registry Hive", |ui| {
                    egui::Grid::new("startup_details_grid")
                        .min_col_width(170.0)
                        .show(ui, |ui| {
                            ui.label("Registry Key:");
                            ui.label(
                                RichText::new(
                                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                                )
                                .monospace()
                                .small(),
                            );
                            ui.end_row();

                            ui.label("Registry Value:");
                            ui.label(RichText::new("Theasus").monospace().small());
                            ui.end_row();

                            ui.label("Registered Command:");
                            let cmd_str = state
                                .startup_status
                                .command
                                .as_deref()
                                .unwrap_or("(Not configured)");
                            let color = if state.startup_status.enabled {
                                Theme::ACCENT_CYAN
                            } else {
                                Theme::TEXT_MUTED
                            };
                            ui.label(RichText::new(cmd_str).monospace().color(color).small());
                            ui.end_row();

                            ui.label("Detected Target Path:");
                            let target_str = state
                                .startup_status
                                .detected_exe
                                .as_ref()
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or_else(|| "(Unknown)".to_string());
                            ui.label(RichText::new(target_str).monospace().small());
                            ui.end_row();

                            ui.label("Privilege Level:");
                            ui.label(
                                RichText::new("Standard User (No UAC administrator elevation prompt needed)")
                                    .color(Theme::ACCENT_GREEN)
                                    .small(),
                            );
                            ui.end_row();
                        });

                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(
                            "• Sleep/Wake Behavior: If your PC sleeps, Theasus stays active in memory and reconnects HID devices via hotplug polling.\n• Restart/Shutdown Behavior: Windows launches Theasus upon login and initializes low-level hooks instantly.",
                        )
                        .color(Theme::TEXT_MUTED)
                        .small(),
                    );
                });
            });
        });

        ui.add_space(12.0);

        // Windows Start Menu & Search Integration Card
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("🔍 Windows Start Menu & Search Integration")
                            .strong()
                            .size(14.0),
                    );
                    ui.separator();
                    if state.shortcut_installed {
                        ui.label(
                            RichText::new("● INSTALLED (SEARCHABLE)")
                                .color(Theme::ACCENT_GREEN)
                                .strong()
                                .size(11.0),
                        );
                    } else {
                        ui.label(
                            RichText::new("○ NOT INSTALLED")
                                .color(Theme::ACCENT_AMBER)
                                .strong()
                                .size(11.0),
                        );
                    }
                });

                ui.add_space(4.0);
                ui.label(
                    RichText::new(
                        "Registers Theasus into the Windows Start Menu, All Apps list, and Windows Search (Win key / Win + S). You can immediately search 'Theasus' or 'thesus' from your taskbar to open it directly.",
                    )
                    .color(Theme::TEXT_SECONDARY)
                    .small(),
                );

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("⭐ Install / Repair Start Menu Shortcuts").clicked() {
                        state.install_shortcut();
                    }

                    if state.shortcut_installed {
                        if ui.button("🗑 Remove from Start Menu").clicked() {
                            state.uninstall_shortcut();
                        }
                    }
                });

                ui.add_space(6.0);
                ui.label(
                    RichText::new(
                        "• Search Aliases: Matches 'Theasus' and 'Thesus'\n• Shortcut Location: %APPDATA%\\Microsoft\\Windows\\Start Menu\\Programs\\Theasus.lnk\n• App Paths: HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\App Paths\\theasus.exe (Supports Win+R)",
                    )
                    .color(Theme::TEXT_MUTED)
                    .small(),
                );
            });
        });

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
                    ui.label(
                        RichText::new("Windows Raw Input (hDevice Keystroke Resolution)")
                            .monospace()
                            .color(Theme::ACCENT_GREEN),
                    );
                    ui.end_row();

                    ui.label("Input Interception:");
                    ui.label(
                        RichText::new("Windows WH_KEYBOARD_LL (Microsecond Worker)").monospace(),
                    );
                    ui.end_row();

                    ui.label("Anti-Recursion Tag:");
                    ui.label(
                        RichText::new(format!("0x{:08X} (\"KCC1\")", KCC_MAGIC_EXTRA_INFO))
                            .monospace()
                            .color(Theme::ACCENT_CYAN),
                    );
                    ui.end_row();

                    ui.label("Configuration Directory:");
                    ui.label(
                        RichText::new(state.storage.get_storage_path().to_string_lossy())
                            .monospace()
                            .small(),
                    );
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
                ui.label(
                    RichText::new(
                        "Reverts custom profiles and mappings to baseline configurations.",
                    )
                    .color(Theme::TEXT_SECONDARY)
                    .small(),
                );

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
