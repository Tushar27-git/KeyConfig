pub mod state;
pub mod theme;

use crate::app::state::AppState;
use crate::app::theme::Theme;
use crate::input::normalize::InputEvent;
use crate::ui::MainWindow;
use crossbeam_channel::Receiver;

use crate::profiles::model::Profile;
use parking_lot::RwLock;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

pub struct KeyboardApp {
    state: AppState,
    window: MainWindow,
}

impl KeyboardApp {
    pub fn new(
        event_rx: Receiver<InputEvent>,
        device_rx: Receiver<String>,
        capture_active: Arc<AtomicBool>,
        active_profile_arc: Arc<RwLock<Profile>>,
        cc: &eframe::CreationContext<'_>,
    ) -> Self {
        Theme::apply(&cc.egui_ctx);

        Self {
            state: AppState::new(event_rx, device_rx, capture_active, active_profile_arc),
            window: MainWindow::new(),
        }
    }
}

impl eframe::App for KeyboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Drain incoming events from Raw Input and Low-Level Hook threads
        let had_activity = self.state.process_incoming_events();

        // If activity arrived, capturing, or keys are pressed, request immediate redraw for instant responsiveness
        if had_activity || self.state.capture_state.is_listening() || !self.state.pressed_keys.is_empty() {
            ctx.request_repaint();
        } else {
            // Keep background timer alive at smooth 30-60 FPS without high CPU usage
            ctx.request_repaint_after(Duration::from_millis(33));
        }

        // Render UI
        self.window.render(ctx, &mut self.state);
    }
}
