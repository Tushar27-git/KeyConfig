use crate::input::keys::VKey;
use crate::input::normalize::{InputEvent, InputOrigin, KeyState};
use crate::profiles::model::Profile;
use crate::remap::RemapEngine;
use chrono::Local;
use crossbeam_channel::Sender;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, PostThreadMessageW, SetWindowsHookExW,
    TranslateMessage, UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN,
    WM_KEYUP, WM_QUIT, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

const LLKHF_INJECTED: u32 = 0x00000001;

struct HookGlobalState {
    event_tx: Sender<InputEvent>,
    active_profile: Arc<RwLock<Profile>>,
    start_instant: Instant,
}

static mut HOOK_STATE: Option<HookGlobalState> = None;
static HOOK_HANDLE: AtomicU32 = AtomicU32::new(0);
static HOOK_THREAD_ID: AtomicU32 = AtomicU32::new(0);

pub struct InputHookSupervisor {
    _thread_handle: std::thread::JoinHandle<()>,
}

impl InputHookSupervisor {
    pub fn start(
        event_tx: Sender<InputEvent>,
        active_profile: Arc<RwLock<Profile>>,
    ) -> Result<Self, anyhow::Error> {
        let (init_tx, init_rx) = std::sync::mpsc::channel();

        let thread_handle = std::thread::Builder::new()
            .name("windows-input-hook-worker".to_string())
            .spawn(move || {
                let thread_id = unsafe { windows::Win32::System::Threading::GetCurrentThreadId() };
                HOOK_THREAD_ID.store(thread_id, Ordering::SeqCst);

                unsafe {
                    HOOK_STATE = Some(HookGlobalState {
                        event_tx,
                        active_profile,
                        start_instant: Instant::now(),
                    });

                    let hook = SetWindowsHookExW(
                        WH_KEYBOARD_LL,
                        Some(low_level_keyboard_proc),
                        None,
                        0,
                    );

                    match hook {
                        Ok(hhook) => {
                            HOOK_HANDLE.store(hhook.0 as usize as u32, Ordering::SeqCst);
                            let _ = init_tx.send(Ok(()));

                            let mut msg = MSG::default();
                            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                                if msg.message == WM_QUIT {
                                    break;
                                }
                                let _ = TranslateMessage(&msg);
                                DispatchMessageW(&msg);
                            }

                            let _ = UnhookWindowsHookEx(hhook);
                            HOOK_HANDLE.store(0, Ordering::SeqCst);
                            HOOK_STATE = None;
                        }
                        Err(e) => {
                            let _ = init_tx.send(Err(anyhow::anyhow!(
                                "Failed to install WH_KEYBOARD_LL hook: {:?}",
                                e
                            )));
                        }
                    }
                }
            })?;

        init_rx.recv()??;

        Ok(Self {
            _thread_handle: thread_handle,
        })
    }

    pub fn stop(&self) {
        let thread_id = HOOK_THREAD_ID.load(Ordering::SeqCst);
        if thread_id != 0 {
            unsafe {
                let _ = PostThreadMessageW(thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
            }
        }
    }
}

unsafe extern "system" fn low_level_keyboard_proc(
    ncode: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if ncode >= 0 {
        let msg = wparam.0 as u32;
        let is_down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
        let is_up = msg == WM_KEYUP || msg == WM_SYSKEYUP;

        if is_down || is_up {
            let state = if is_down {
                KeyState::Down
            } else {
                KeyState::Up
            };
            let kbd = *(lparam.0 as *const KBDLLHOOKSTRUCT);
            let raw_vk = kbd.vkCode as u16;
            let scan_code = kbd.scanCode;
            let is_injected = (kbd.flags.0 & LLKHF_INJECTED) != 0;
            let origin = if is_injected {
                InputOrigin::Injected
            } else {
                InputOrigin::Physical
            };
            let extra_info = kbd.dwExtraInfo;

            if let Some(ref state_ctx) = HOOK_STATE {
                let current_micros = state_ctx.start_instant.elapsed().as_micros() as u64;
                let (action, intercept) = {
                    let profile_guard = state_ctx.active_profile.read();
                    RemapEngine::process_keystroke(
                        VKey::from_vk(raw_vk),
                        state,
                        origin,
                        extra_info,
                        &profile_guard,
                    )
                };

                let event = InputEvent {
                    timestamp: Local::now(),
                    instant_micros: current_micros,
                    vkey: VKey::from_vk(raw_vk),
                    raw_vk,
                    scan_code,
                    state,
                    origin,
                    action,
                    device_handle_raw: 0,
                };

                let _ = state_ctx.event_tx.try_send(event);

                if intercept {
                    return LRESULT(1);
                }
            }
        }
    }

    let raw_hook = HOOK_HANDLE.load(Ordering::SeqCst);
    let hhook = if raw_hook != 0 {
        Some(HHOOK(raw_hook as usize as _))
    } else {
        None
    };
    CallNextHookEx(hhook, ncode, wparam, lparam)
}
