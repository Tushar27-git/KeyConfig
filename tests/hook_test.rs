use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use parking_lot::RwLock;
use theasus::input::hook::InputHookSupervisor;
use theasus::profiles::model::Profile;

#[test]
fn test_hook_supervisor_lifecycle() {
    let (event_tx, _event_rx) = crossbeam_channel::bounded(100);
    let active_profile = Arc::new(RwLock::new(Profile::new_default()));
    let capture_active = Arc::new(AtomicBool::new(false));

    let supervisor = InputHookSupervisor::start(event_tx, active_profile, capture_active);
    assert!(supervisor.is_ok(), "InputHookSupervisor failed to start: {:?}", supervisor.err());

    let sup = supervisor.unwrap();
    // Verify stop signal terminates worker cleanly without hanging
    sup.stop();
}
