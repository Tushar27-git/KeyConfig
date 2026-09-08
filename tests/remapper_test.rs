use theasus::input::injector::KCC_MAGIC_EXTRA_INFO;
use theasus::input::keys::VKey;
use theasus::input::normalize::{InputOrigin, KeyState, RemapAction};
use theasus::profiles::model::Profile;
use theasus::remap::rules::MappingTarget;
use theasus::remap::RemapEngine;

#[test]
fn test_anti_recursion_guard() {
    let profile = Profile::new_default();

    // 1. Self-injected event with magic tag must NEVER be remapped
    let (action, intercept) = RemapEngine::process_keystroke(
        VKey::KeyA,
        KeyState::Down,
        InputOrigin::Injected,
        KCC_MAGIC_EXTRA_INFO,
        &profile,
    );
    assert_eq!(action, RemapAction::SelfInjected);
    assert!(!intercept, "Self-injected input must not be intercepted");

    // 2. Generic injected event must pass through
    let (action, intercept) = RemapEngine::process_keystroke(
        VKey::KeyA,
        KeyState::Down,
        InputOrigin::Injected,
        0,
        &profile,
    );
    assert_eq!(action, RemapAction::PassThrough);
    assert!(!intercept, "External injected input must pass through");
}

#[test]
fn test_key_remapping_resolution() {
    let mut profile = Profile::new_default();
    profile.mappings.insert(VKey::CapsLock, MappingTarget::Key(VKey::Escape));

    // Physical CapsLock key down
    let (action, intercept) = RemapEngine::process_keystroke(
        VKey::CapsLock,
        KeyState::Down,
        InputOrigin::Physical,
        0,
        &profile,
    );
    assert_eq!(action, RemapAction::Remapped { target: VKey::Escape });
    assert!(intercept, "Remapped key must intercept physical event");

    // Unmapped key must pass through
    let (action, intercept) = RemapEngine::process_keystroke(
        VKey::KeyA,
        KeyState::Down,
        InputOrigin::Physical,
        0,
        &profile,
    );
    assert_eq!(action, RemapAction::PassThrough);
    assert!(!intercept, "Unmapped key must not intercept");
}

#[test]
fn test_key_blocking_resolution() {
    let mut profile = Profile::new_default();
    profile.mappings.insert(VKey::WinLeft, MappingTarget::Block);

    let (action, intercept) = RemapEngine::process_keystroke(
        VKey::WinLeft,
        KeyState::Down,
        InputOrigin::Physical,
        0,
        &profile,
    );
    assert_eq!(action, RemapAction::Blocked);
    assert!(intercept, "Blocked key must intercept physical event");
}
