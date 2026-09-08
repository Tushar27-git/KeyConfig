use crate::input::injector::{InputInjector, KCC_MAGIC_EXTRA_INFO};
use crate::input::keys::VKey;
use crate::input::normalize::{InputOrigin, KeyState, RemapAction};
use crate::profiles::model::Profile;
use crate::remap::rules::MappingTarget;

pub struct RemapEngine;

impl RemapEngine {
    /// Evaluates an incoming keystroke against the active profile.
    ///
    /// # Critical Anti-Recursion Safety:
    /// Injected events (tagged with KCC_MAGIC_EXTRA_INFO or flagged as Injected by the OS)
    /// must NEVER be remapped and immediately bypass this resolver.
    pub fn process_keystroke(
        vkey: VKey,
        state: KeyState,
        _origin: InputOrigin,
        extra_info: usize,
        active_profile: &Profile,
    ) -> (RemapAction, bool) {
        // 1. Anti-recursion guard: synthesized keystrokes from Theasus carry our magic extra info tag
        if extra_info == KCC_MAGIC_EXTRA_INFO {
            return (RemapAction::SelfInjected, false);
        }

        // 2. Physical keystroke lookup
        if let Some(target) = active_profile.mappings.get(&vkey) {
            match target {
                MappingTarget::Key(dest_vkey) => {
                    InputInjector::inject_key(*dest_vkey, state);
                    (RemapAction::Remapped { target: *dest_vkey }, true)
                }
                MappingTarget::Block => (RemapAction::Blocked, true),
            }
        } else {
            (RemapAction::PassThrough, false)
        }
    }
}
