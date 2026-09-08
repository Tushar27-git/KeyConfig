use crate::device::identity::RawDeviceResolver;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::UI::Input::{
    GetRawInputData, RegisterRawInputDevices, HRAWINPUT, RAWINPUT, RAWINPUTDEVICE,
    RAWINPUTHEADER, RIDEV_DEVNOTIFY, RIDEV_INPUTSINK, RID_INPUT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW,
    PostThreadMessageW, RegisterClassW, TranslateMessage, HWND_MESSAGE, MSG, WINDOW_EX_STYLE,
    WM_INPUT, WM_INPUT_DEVICE_CHANGE, WM_QUIT, WNDCLASSW,
};

pub struct RawInputWorker {
    _thread_handle: std::thread::JoinHandle<()>,
    thread_id: u32,
}

static RAW_INPUT_THREAD_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

// Global sender for device path strings from Raw Input
static mut RAW_DEVICE_NOTIFIER: Option<crossbeam_channel::Sender<String>> = None;
static mut HOTPLUG_NOTIFIER: Option<Arc<AtomicBool>> = None;

impl RawInputWorker {
    pub fn start(
        device_tx: crossbeam_channel::Sender<String>,
        hotplug_trigger: Arc<AtomicBool>,
    ) -> Result<Self, anyhow::Error> {
        let (init_tx, init_rx) = std::sync::mpsc::channel();

        let thread_handle = std::thread::Builder::new()
            .name("windows-raw-input-worker".to_string())
            .spawn(move || {
                let thread_id = unsafe { windows::Win32::System::Threading::GetCurrentThreadId() };
                RAW_INPUT_THREAD_ID.store(thread_id, Ordering::SeqCst);

                unsafe {
                    RAW_DEVICE_NOTIFIER = Some(device_tx);
                    HOTPLUG_NOTIFIER = Some(hotplug_trigger);

                    let class_name = w!("KCC_RawInput_Class");
                    let wnd_class = WNDCLASSW {
                        lpfnWndProc: Some(raw_input_wnd_proc),
                        lpszClassName: class_name,
                        hbrBackground: HBRUSH(std::ptr::null_mut()),
                        ..Default::default()
                    };

                    let _ = RegisterClassW(&wnd_class);

                    // Create message-only window
                    let hwnd = CreateWindowExW(
                        WINDOW_EX_STYLE(0),
                        class_name,
                        w!("KCC_RawInput_Window"),
                        windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE(0),
                        0,
                        0,
                        0,
                        0,
                        Some(HWND_MESSAGE),
                        None,
                        None,
                        None,
                    );

                    match hwnd {
                        Ok(wnd) => {
                            // Register for Raw Input on Keyboard (Page 0x01, Usage 0x06)
                            let rid = RAWINPUTDEVICE {
                                usUsagePage: 0x01, // Generic Desktop Controls
                                usUsage: 0x06,     // Keyboard
                                dwFlags: RIDEV_INPUTSINK | RIDEV_DEVNOTIFY,
                                hwndTarget: wnd,
                            };

                            let registered = RegisterRawInputDevices(
                                &[rid],
                                std::mem::size_of::<RAWINPUTDEVICE>() as u32,
                            );

                            if let Err(e) = registered {
                                let _ = init_tx.send(Err(anyhow::anyhow!(
                                    "Failed to RegisterRawInputDevices: {:?}",
                                    e
                                )));
                                return;
                            }

                            let _ = init_tx.send(Ok(thread_id));

                            // Message pump
                            let mut msg = MSG::default();
                            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                                if msg.message == WM_QUIT {
                                    break;
                                }
                                let _ = TranslateMessage(&msg);
                                DispatchMessageW(&msg);
                            }

                            let _ = DestroyWindow(wnd);
                        }
                        Err(e) => {
                            let _ = init_tx.send(Err(anyhow::anyhow!(
                                "Failed to create raw input message window: {:?}",
                                e
                            )));
                        }
                    }
                }
            })?;

        let thread_id = init_rx.recv()??;

        Ok(Self {
            _thread_handle: thread_handle,
            thread_id,
        })
    }

    pub fn stop(&self) {
        if self.thread_id != 0 {
            unsafe {
                let _ = PostThreadMessageW(self.thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
            }
        }
    }
}

unsafe extern "system" fn raw_input_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_INPUT => {
            let mut size: u32 = 0;
            let _ = GetRawInputData(
                HRAWINPUT(lparam.0 as _),
                RID_INPUT,
                None,
                &mut size,
                std::mem::size_of::<RAWINPUTHEADER>() as u32,
            );

            if size > 0 {
                let mut buffer: Vec<u8> = vec![0; size as usize];
                let read = GetRawInputData(
                    HRAWINPUT(lparam.0 as _),
                    RID_INPUT,
                    Some(buffer.as_mut_ptr() as *mut _),
                    &mut size,
                    std::mem::size_of::<RAWINPUTHEADER>() as u32,
                );

                if read != u32::MAX && read > 0 {
                    let raw = *(buffer.as_ptr() as *const RAWINPUT);
                    let h_device = raw.header.hDevice;

                    if !h_device.is_invalid() {
                        if let Some(path) = RawDeviceResolver::get_device_path(h_device) {
                            if let Some(ref tx) = RAW_DEVICE_NOTIFIER {
                                let _ = tx.try_send(path);
                            }
                        }
                    }
                }
            }
            LRESULT(0)
        }
        WM_INPUT_DEVICE_CHANGE => {
            if let Some(ref hotplug) = HOTPLUG_NOTIFIER {
                hotplug.store(true, Ordering::SeqCst);
            }
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
