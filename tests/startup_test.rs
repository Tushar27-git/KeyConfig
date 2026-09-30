use theasus::startup::{StartupLaunchMode, StartupManager};

#[test]
fn test_startup_manager_query() {
    let status = StartupManager::query_status();
    assert!(status.is_ok(), "Querying startup status should succeed");
    let s = status.unwrap();
    println!("Startup status: enabled={}, mode={:?}", s.enabled, s.launch_mode);
}

#[test]
fn test_startup_manager_lifecycle() {
    // Record initial status
    let initial = StartupManager::query_status().expect("query status");

    // Enable in Minimized mode
    let enable_res = StartupManager::enable(StartupLaunchMode::Minimized);
    assert!(enable_res.is_ok(), "Enabling startup should succeed");

    let after_enable = StartupManager::query_status().expect("query status after enable");
    assert!(after_enable.enabled, "Startup should now be enabled");
    assert_eq!(after_enable.launch_mode, StartupLaunchMode::Minimized);
    assert!(after_enable.command.is_some());
    assert!(after_enable.command.unwrap().contains("--minimized"));

    // Enable in Normal mode
    let enable_norm = StartupManager::enable(StartupLaunchMode::Normal);
    assert!(enable_norm.is_ok(), "Enabling startup normal should succeed");
    let after_norm = StartupManager::query_status().expect("query status after normal");
    assert_eq!(after_norm.launch_mode, StartupLaunchMode::Normal);

    // Disable
    let disable_res = StartupManager::disable();
    assert!(disable_res.is_ok(), "Disabling startup should succeed");

    let after_disable = StartupManager::query_status().expect("query status after disable");
    assert!(!after_disable.enabled, "Startup should now be disabled");

    // Restore original status if it was previously enabled
    if initial.enabled {
        let _ = StartupManager::enable(initial.launch_mode);
    }
}

#[test]
fn test_shortcut_manager_query() {
    let installed = theasus::startup::ShortcutManager::is_installed();
    println!("Shortcut installed: {}", installed);
    assert!(installed, "Start menu shortcut should be detected");
}

