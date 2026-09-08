pub mod state;
pub mod theme;

use crate::app::state::AppState;
use crate::app::theme::Theme;
use crate::input::normalize::InputEvent;
use crate::ui::MainWindow;
use crossbeam_channel::Receiver;

pub struct KeyboardApp {
    state: AppState,
    window: MainWindow,
}

impl KeyboardApp {
    pub fn new(
        event_rx: Receiver<InputEvent>,
        device_rx: Receiver<String>,
        cc: &eframe::CreationContext<'_>,
    ) -> Self {
        Theme::apply(&cc.egui_ctx);

        Self {
            state: AppState::new(event_rx, device_rx),
            window: MainWindow::new(),
        }
    }
}

impl eframe::App for KeyboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Drain incoming events from Raw Input and Low-Level Hook threads
        self.state.process_incoming_events();

        // Render UI
        self.window.render(ctx, &mut self.state);
    }
}
