use crate::app::state::AppState;
use crate::app::theme::Theme;
use crate::diagnostics::EventFilter;
use crate::input::normalize::{KeyState, RemapAction};
use egui::{RichText, Ui};

pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("Input Diagnostics & Latency Monitor").size(18.0).color(Theme::TEXT_PRIMARY));
        ui.add_space(8.0);

        // Observed Input Rate Panel
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Observed Input Rate (Host Measured)").strong().size(14.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Reset Stats").clicked() {
                            state.rate_calc.reset();
                        }
                    });
                });

                ui.add_space(6.0);

                let stats = &state.rate_stats;

                ui.columns(5, |cols| {
                    cols[0].label(RichText::new("Observed Rate").color(Theme::TEXT_MUTED).small());
                    cols[0].label(RichText::new(format!("{:.1} Hz", stats.observed_hz)).strong().size(18.0).color(Theme::ACCENT_CYAN));

                    cols[1].label(RichText::new("Median Interval").color(Theme::TEXT_MUTED).small());
                    cols[1].label(RichText::new(format!("{:.2} ms", stats.median_ms)).strong().size(18.0));

                    cols[2].label(RichText::new("Average Interval").color(Theme::TEXT_MUTED).small());
                    cols[2].label(RichText::new(format!("{:.2} ms", stats.avg_ms)).strong().size(18.0));

                    cols[3].label(RichText::new("Jitter (StdDev)").color(Theme::TEXT_MUTED).small());
                    cols[3].label(RichText::new(format!("±{:.2} ms", stats.jitter_ms)).strong().size(18.0).color(Theme::ACCENT_AMBER));

                    cols[4].label(RichText::new("Samples").color(Theme::TEXT_MUTED).small());
                    cols[4].label(RichText::new(format!("{}", stats.samples)).strong().size(18.0));
                });

                ui.add_space(6.0);

                // Polling Rate Reality Callout
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Hardware Polling Control:").color(Theme::TEXT_MUTED).small());
                    ui.label(RichText::new("Not exposed by this device (Host measurement only; true hardware polling rate requires verified vendor firmware protocol).")
                        .color(Theme::TEXT_SECONDARY).small());
                });
            });
        });

        ui.add_space(12.0);

        // Live Input Monitor Controls
        ui.horizontal(|ui| {
            ui.heading(RichText::new("Live Input Event Monitor").size(15.0));

            ui.separator();

            if state.event_log.is_paused {
                if ui.button("▶ Resume Monitor").clicked() {
                    state.event_log.is_paused = false;
                }
            } else if ui.button("⏸ Pause Monitor").clicked() {
                state.event_log.is_paused = true;
            }

            if ui.button("🗑 Clear Log").clicked() {
                state.event_log.clear();
            }

            if ui.button("📋 Copy to Clipboard").clicked() {
                let text = state.event_log.to_clipboard_text();
                ui.ctx().copy_text(text);
                state.set_status("Copied event log to clipboard");
            }

            ui.separator();

            // Filter combo
            ui.label("Filter:");
            egui::ComboBox::from_id_salt("event_filter")
                .selected_text(match state.event_log.filter {
                    EventFilter::All => "All Events",
                    EventFilter::PhysicalOnly => "Physical Only",
                    EventFilter::InjectedOnly => "Injected Only",
                    EventFilter::RemappedOnly => "Remapped Only",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.event_log.filter, EventFilter::All, "All Events");
                    ui.selectable_value(&mut state.event_log.filter, EventFilter::PhysicalOnly, "Physical Only");
                    ui.selectable_value(&mut state.event_log.filter, EventFilter::InjectedOnly, "Injected Only");
                    ui.selectable_value(&mut state.event_log.filter, EventFilter::RemappedOnly, "Remapped Only");
                });
        });

        ui.add_space(6.0);

        // Live Event Stream Table
        let filtered = state.event_log.filtered_events();

        egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
            ui.set_width(ui.available_width());

            // Header row
            ui.horizontal(|ui| {
                ui.label(RichText::new("Timestamp").strong().color(Theme::TEXT_MUTED).monospace());
                ui.add_space(30.0);
                ui.label(RichText::new("State").strong().color(Theme::TEXT_MUTED));
                ui.add_space(20.0);
                ui.label(RichText::new("Key (Virtual / Code)").strong().color(Theme::TEXT_MUTED));
                ui.add_space(40.0);
                ui.label(RichText::new("Scan Code").strong().color(Theme::TEXT_MUTED).monospace());
                ui.add_space(30.0);
                ui.label(RichText::new("Action Taken").strong().color(Theme::TEXT_MUTED));
            });
            ui.separator();

            if filtered.is_empty() {
                ui.label(RichText::new("Waiting for keyboard events... Type on your keyboard to monitor events.").color(Theme::TEXT_MUTED));
            } else {
                for ev in filtered.iter().rev().take(100) {
                    ui.horizontal(|ui| {
                        // Timestamp
                        ui.label(RichText::new(ev.timestamp.format("%H:%M:%S%.3f").to_string()).color(Theme::TEXT_SECONDARY).monospace());

                        // State
                        match ev.state {
                            KeyState::Down => {
                                ui.label(RichText::new("KeyDown").color(Theme::ACCENT_AMBER).strong());
                            }
                            KeyState::Up => {
                                ui.label(RichText::new("KeyUp  ").color(Theme::TEXT_MUTED));
                            }
                        }

                        // Key
                        ui.label(RichText::new(format!("{:?} (0x{:02X})", ev.vkey, ev.raw_vk)).monospace());

                        // Scan Code
                        ui.label(RichText::new(format!("0x{:04X}", ev.scan_code)).color(Theme::TEXT_MUTED).monospace());

                        // Action
                        match &ev.action {
                            RemapAction::PassThrough => {
                                ui.label(RichText::new("PassThrough").color(Theme::ACCENT_GREEN));
                            }
                            RemapAction::Remapped { target } => {
                                ui.label(RichText::new(format!("→ Remapped to {:?}", target)).color(Theme::ACCENT_CYAN).strong());
                            }
                            RemapAction::Blocked => {
                                ui.label(RichText::new("✕ Blocked").color(Theme::ACCENT_RED).strong());
                            }
                            RemapAction::SelfInjected => {
                                ui.label(RichText::new("⚡ Injected (App)").color(Theme::TEXT_MUTED));
                            }
                        }
                    });
                }
            }
        });
    });
}
