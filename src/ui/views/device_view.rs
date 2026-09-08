use crate::app::state::AppState;
use crate::app::theme::Theme;
use egui::{RichText, Ui};

pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.heading(RichText::new("Device Inspector & Auto-Detection").size(18.0).color(Theme::TEXT_PRIMARY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🔄 Refresh Devices").clicked() {
                    state.device_manager.refresh();
                    state.set_status("Refreshed device list");
                }
            });
        });
        ui.add_space(8.0);

        // Active Keyboard Card (Identified by Keystroke via Raw Input hDevice)
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if state.device_manager.active_keyboard().is_some() {
                        ui.label(RichText::new("● ACTIVE (TYPING ON)").color(Theme::ACCENT_GREEN).strong());
                    } else {
                        ui.label(RichText::new("○ WAITING FOR KEYSTROKE").color(Theme::ACCENT_AMBER).strong());
                    }

                    let active_label = state.device_manager.active_keyboard_label();
                    ui.label(RichText::new(active_label).strong().size(15.0));
                });

                ui.add_space(6.0);

                if let Some(active) = state.device_manager.active_keyboard() {
                    ui.columns(4, |cols| {
                        cols[0].label(RichText::new("Vendor ID (VID)").color(Theme::TEXT_MUTED).small());
                        cols[0].label(RichText::new(format!("0x{:04X}", active.vendor_id)).monospace().strong());

                        cols[1].label(RichText::new("Product ID (PID)").color(Theme::TEXT_MUTED).small());
                        cols[1].label(RichText::new(format!("0x{:04X}", active.product_id)).monospace().strong());

                        cols[2].label(RichText::new("Interface / Role").color(Theme::TEXT_MUTED).small());
                        cols[2].label(RichText::new(format!("Interface {}", active.interface_number)).strong());

                        cols[3].label(RichText::new("Identification Method").color(Theme::TEXT_MUTED).small());
                        cols[3].label(RichText::new("Raw Input hDevice").color(Theme::ACCENT_CYAN).strong());
                    });
                    ui.add_space(4.0);
                    ui.label(RichText::new(&active.path).color(Theme::TEXT_MUTED).small().monospace());
                } else {
                    ui.label(RichText::new("Press any key on any connected keyboard to immediately identify its physical hardware handle.").color(Theme::TEXT_SECONDARY));
                }
            });
        });

        ui.add_space(12.0);

        // Hardware Capability Status Matrix
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.label(RichText::new("Hardware Capability Status Matrix").strong().size(14.0));
                ui.label(RichText::new("Strictly distinguishes host-level capabilities from unverified hardware features.")
                    .color(Theme::TEXT_SECONDARY).small());

                ui.add_space(6.0);

                let caps = state.device_manager.capabilities();

                egui::Grid::new("caps_grid").striped(true).min_col_width(200.0).show(ui, |ui| {
                    ui.label(RichText::new("Capability").strong().color(Theme::TEXT_MUTED));
                    ui.label(RichText::new("Status").strong().color(Theme::TEXT_MUTED));
                    ui.label(RichText::new("Execution Domain").strong().color(Theme::TEXT_MUTED));
                    ui.end_row();

                    ui.label("Software Key Remapping");
                    ui.label(RichText::new(caps.software_remap.label()).color(Theme::ACCENT_GREEN).strong());
                    ui.label("Host OS (WH_KEYBOARD_LL)");
                    ui.end_row();

                    ui.label("Identify-by-Keystroke Auto-Detection");
                    ui.label(RichText::new("CONFIRMED (HOST)").color(Theme::ACCENT_GREEN).strong());
                    ui.label("Windows Raw Input (WM_INPUT)");
                    ui.end_row();

                    ui.label("Observed Latency / Jitter Diagnostics");
                    ui.label(RichText::new("CONFIRMED (HOST)").color(Theme::ACCENT_GREEN).strong());
                    ui.label("Host OS (Microsecond Timestamp)");
                    ui.end_row();

                    ui.label("Hardware Polling-Rate Control");
                    ui.label(RichText::new(caps.polling_rate_control.label()).color(Theme::TEXT_MUTED));
                    ui.label("Not Exposed by Device");
                    ui.end_row();

                    ui.label("Onboard Flash Keymaps");
                    ui.label(RichText::new(caps.hardware_remap.label()).color(Theme::ACCENT_AMBER));
                    ui.label("Requires Verified Protocol");
                    ui.end_row();

                    ui.label("Hardware RGB Control");
                    ui.label(RichText::new(caps.rgb_control.label()).color(Theme::ACCENT_AMBER));
                    ui.label("Quarantined (Read-Only)");
                    ui.end_row();
                });
            });
        });

        ui.add_space(12.0);

        // Detected Keyboards (Phase 1a Passive Enumeration)
        ui.horizontal(|ui| {
            ui.heading(RichText::new("Detected Physical Keyboards & HID Interfaces").size(15.0));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("📋 Copy Device Info").clicked() {
                    let mut text = String::new();
                    for dev in state.device_manager.detected_keyboards() {
                        text.push_str(&format!(
                            "VID: 0x{:04X} | PID: 0x{:04X} | IF: {} | Page: 0x{:04X} | Usage: 0x{:04X} | Name: {:?} | Path: {}\n",
                            dev.vendor_id, dev.product_id, dev.interface_number, dev.usage_page, dev.usage, dev.product_name, dev.path
                        ));
                    }
                    ui.ctx().copy_text(text);
                    state.set_status("Copied device list to clipboard");
                }
            });
        });

        egui::ScrollArea::vertical().show(ui, |ui| {
            for dev in state.device_manager.detected_keyboards() {
                let is_active = state.device_manager.active_keyboard().map(|a| a.path == dev.path).unwrap_or(false);

                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        if is_active {
                            ui.label(RichText::new("★ ACTIVE").color(Theme::ACCENT_GREEN).strong());
                        } else if dev.is_cbg_k26 {
                            ui.label(RichText::new("● CBG K26").color(Theme::ACCENT_CYAN).strong());
                        } else {
                            ui.label(RichText::new("  HID").color(Theme::TEXT_MUTED));
                        }

                        ui.vertical(|ui| {
                            ui.label(RichText::new(dev.display_name()).strong());
                            ui.label(RichText::new(format!(
                                "VID: 0x{:04X} | PID: 0x{:04X} | Interface: {} | Usage Page: 0x{:04X} | Usage: 0x{:04X}",
                                dev.vendor_id, dev.product_id, dev.interface_number, dev.usage_page, dev.usage
                            )).color(Theme::TEXT_SECONDARY).small().monospace());
                            ui.label(RichText::new(&dev.path).color(Theme::TEXT_MUTED).small().monospace());
                        });
                    });
                });
                ui.add_space(4.0);
            }
        });
    });
}
