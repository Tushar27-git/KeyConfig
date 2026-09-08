#![windows_subsystem = "windows"]

use parking_lot::RwLock;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use theasus::app::KeyboardApp;
use theasus::input::hook::InputHookSupervisor;
use theasus::input::raw_input::RawInputWorker;
use theasus::profiles::model::Profile;

fn main() -> Result<(), eframe::Error> {
    // Detach from any parent console immediately so no background CMD window lingers
    #[cfg(windows)]
    unsafe {
        let _ = windows::Win32::System::Console::FreeConsole();
    }

    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Starting Theasus Keyboard Control Center (Raw Input Auto-Detection)...");

    // Channels for low-level input events and physical device path strings
    let (event_tx, event_rx) = crossbeam_channel::bounded(2048);
    let (device_tx, device_rx) = crossbeam_channel::bounded(256);

    let hotplug_trigger = Arc::new(AtomicBool::new(false));

    // Spawn Raw Input Worker (Windows WM_INPUT + WM_INPUT_DEVICE_CHANGE)
    let raw_input_worker = match RawInputWorker::start(device_tx, hotplug_trigger.clone()) {
        Ok(worker) => {
            tracing::info!("Raw Input Worker thread successfully spawned");
            Some(worker)
        }
        Err(e) => {
            tracing::error!("Failed to spawn Raw Input Worker: {:?}", e);
            None
        }
    };

    // Shared active profile for remapping engine (SHARED WITH HOOK WORKER!)
    let active_profile_arc = Arc::new(RwLock::new(Profile::new_default()));

    // Shared capture active flag (disables OS key propagation during physical capture)
    let capture_active = Arc::new(AtomicBool::new(false));

    // Spawn Low-Level Input Interception Hook (WH_KEYBOARD_LL)
    let hook_supervisor = match InputHookSupervisor::start(
        event_tx,
        active_profile_arc.clone(),
        capture_active.clone(),
    ) {
        Ok(sup) => {
            tracing::info!("Low-level input hook thread successfully spawned");
            Some(sup)
        }
        Err(e) => {
            tracing::error!("Failed to spawn Low-Level Input Hook: {:?}", e);
            None
        }
    };

    // Configure Native Win32 Window
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Theasus — Keyboard Control Center")
            .with_inner_size([1080.0, 740.0])
            .with_min_inner_size([880.0, 560.0]),
        ..Default::default()
    };

    let result = eframe::run_native(
        "theasus",
        native_options,
        Box::new(move |cc| {
            Ok(Box::new(KeyboardApp::new(
                event_rx,
                device_rx,
                capture_active,
                active_profile_arc,
                cc,
            )))
        }),
    );

    // Clean up workers on exit
    if let Some(sup) = hook_supervisor {
        sup.stop();
    }
    if let Some(worker) = raw_input_worker {
        worker.stop();
    }

    result
}
