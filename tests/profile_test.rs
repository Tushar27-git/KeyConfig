use theasus::input::keys::VKey;
use theasus::profiles::model::Profile;
use theasus::remap::rules::MappingTarget;
use std::collections::HashMap;

#[test]
fn test_profile_default_factory() {
    let profile = Profile::new_default();
    assert_eq!(profile.id, "default");
    assert!(profile.is_readonly);
    assert!(profile.mappings.is_empty());
}

#[test]
fn test_profile_gaming_factory() {
    let profile = Profile::new_gaming();
    assert_eq!(profile.id, "gaming");
    assert!(!profile.is_readonly);
    assert_eq!(
        profile.mappings.get(&VKey::WinLeft),
        Some(&MappingTarget::Block)
    );
    assert_eq!(
        profile.mappings.get(&VKey::CapsLock),
        Some(&MappingTarget::Key(VKey::ControlLeft))
    );
}

#[test]
fn test_profile_serialization_roundtrip() {
    let mut mappings = HashMap::new();
    mappings.insert(VKey::CapsLock, MappingTarget::Key(VKey::Escape));
    mappings.insert(VKey::WinLeft, MappingTarget::Block);

    let original = Profile {
        id: "custom_test".to_string(),
        name: "Custom Test".to_string(),
        description: "Test description".to_string(),
        is_readonly: false,
        mappings,
    };

    let serialized = serde_json::to_string(&original).expect("Serialization failed");
    let deserialized: Profile =
        serde_json::from_str(&serialized).expect("Deserialization failed");

    assert_eq!(deserialized.id, original.id);
    assert_eq!(deserialized.name, original.name);
    assert_eq!(deserialized.mappings.len(), 2);
    assert_eq!(
        deserialized.mappings.get(&VKey::CapsLock),
        Some(&MappingTarget::Key(VKey::Escape))
    );
    assert_eq!(
        deserialized.mappings.get(&VKey::WinLeft),
        Some(&MappingTarget::Block)
    );
}
