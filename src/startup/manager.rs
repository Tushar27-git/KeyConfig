use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS};
use windows::Win32::System::Registry::{
    RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW,
    HKEY, HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_SZ, REG_VALUE_TYPE,
};

const RUN_SUBKEY: PCWSTR = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
const APP_REG_NAME: PCWSTR = w!("Theasus");
pub const APP_REG_NAME_STR: &str = "Theasus";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StartupLaunchMode {
    #[default]
    Normal,
    Minimized,
}

impl StartupLaunchMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Minimized => "Minimized",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupStatus {
    pub enabled: bool,
    pub command: Option<String>,
    pub launch_mode: StartupLaunchMode,
    pub is_current_exe: bool,
    pub detected_exe: Option<PathBuf>,
}

pub struct StartupManager;

impl StartupManager {
    /// Detect the target executable path to use for autostart.
    /// Priority:
    /// 1. Current executing binary (`std::env::current_exe()`)
    /// 2. If running via debug/cargo/tests, locate workspace root `Theasus.exe` or `target\release\theasus.exe`
    pub fn get_recommended_executable() -> Result<PathBuf> {
        let current = std::env::current_exe().context("Failed to get current executable path")?;

        let path_str = current.to_string_lossy();
        if path_str.contains("target\\debug") || path_str.contains("deps") {
            let mut search_dir = current.parent();
            while let Some(dir) = search_dir {
                let root_exe = dir.join("Theasus.exe");
                if root_exe.is_file() {
                    return Ok(root_exe);
                }
                let release_exe = dir.join("target").join("release").join("theasus.exe");
                if release_exe.is_file() {
                    return Ok(release_exe);
                }
                search_dir = dir.parent();
            }
        }

        Ok(current)
    }

    /// Query the current startup configuration from the Windows Registry
    pub fn query_status() -> Result<StartupStatus> {
        let recommended_exe = Self::get_recommended_executable().ok();
        let mut hkey = HKEY::default();

        let open_res = unsafe {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                RUN_SUBKEY,
                None,
                KEY_READ,
                &mut hkey,
            )
        };

        if open_res != ERROR_SUCCESS {
            return Ok(StartupStatus {
                enabled: false,
                command: None,
                launch_mode: StartupLaunchMode::Normal,
                is_current_exe: false,
                detected_exe: recommended_exe,
            });
        }

        let mut data_type = REG_VALUE_TYPE(0);
        let mut byte_len: u32 = 0;

        // Query size of the value
        let query_res = unsafe {
            RegQueryValueExW(
                hkey,
                APP_REG_NAME,
                None,
                Some(&mut data_type),
                None,
                Some(&mut byte_len),
            )
        };

        if query_res != ERROR_SUCCESS || byte_len == 0 {
            let _ = unsafe { RegCloseKey(hkey) };
            return Ok(StartupStatus {
                enabled: false,
                command: None,
                launch_mode: StartupLaunchMode::Normal,
                is_current_exe: false,
                detected_exe: recommended_exe,
            });
        }

        // Allocate buffer for UTF-16 characters
        let char_len = (byte_len as usize / 2).max(1);
        let mut buffer: Vec<u16> = vec![0; char_len];

        let read_res = unsafe {
            RegQueryValueExW(
                hkey,
                APP_REG_NAME,
                None,
                Some(&mut data_type),
                Some(buffer.as_mut_ptr() as *mut u8),
                Some(&mut byte_len),
            )
        };

        let _ = unsafe { RegCloseKey(hkey) };

        if read_res != ERROR_SUCCESS {
            return Ok(StartupStatus {
                enabled: false,
                command: None,
                launch_mode: StartupLaunchMode::Normal,
                is_current_exe: false,
                detected_exe: recommended_exe,
            });
        }

        // Convert UTF-16 to String, stripping trailing null characters
        let raw_str = String::from_utf16_lossy(&buffer);
        let cmd = raw_str.trim_matches('\0').trim().to_string();

        let is_minimized = cmd.contains("--minimized");
        let launch_mode = if is_minimized {
            StartupLaunchMode::Minimized
        } else {
            StartupLaunchMode::Normal
        };

        let is_current = if let Some(ref rec) = recommended_exe {
            let rec_str = rec.to_string_lossy().to_lowercase();
            cmd.to_lowercase().contains(&rec_str)
        } else {
            false
        };

        Ok(StartupStatus {
            enabled: true,
            command: Some(cmd),
            launch_mode,
            is_current_exe: is_current,
            detected_exe: recommended_exe,
        })
    }

    /// Enable auto-startup by writing to HKCU\Software\Microsoft\Windows\CurrentVersion\Run
    pub fn enable(mode: StartupLaunchMode) -> Result<()> {
        let exe_path = Self::get_recommended_executable()?;
        let exe_str = exe_path.to_string_lossy();

        let mut cmd = format!("\"{}\"", exe_str);
        cmd.push_str(" --autostart");
        if mode == StartupLaunchMode::Minimized {
            cmd.push_str(" --minimized");
        }

        let mut hkey = HKEY::default();
        unsafe {
            let open_res = RegOpenKeyExW(
                HKEY_CURRENT_USER,
                RUN_SUBKEY,
                None,
                KEY_WRITE,
                &mut hkey,
            );

            if open_res != ERROR_SUCCESS {
                return Err(anyhow!("Failed to open Run registry key: {:?}", open_res));
            }

            let mut utf16_cmd: Vec<u16> = cmd.encode_utf16().collect();
            utf16_cmd.push(0); // null terminator

            let byte_slice = std::slice::from_raw_parts(
                utf16_cmd.as_ptr() as *const u8,
                utf16_cmd.len() * 2,
            );

            let set_res = RegSetValueExW(
                hkey,
                APP_REG_NAME,
                None,
                REG_SZ,
                Some(byte_slice),
            );

            let _ = RegCloseKey(hkey);

            if set_res != ERROR_SUCCESS {
                return Err(anyhow!("Failed to set Run registry value: {:?}", set_res));
            }
        }

        tracing::info!("Auto-startup registered successfully: {}", cmd);
        Ok(())
    }

    /// Disable auto-startup by deleting the registry value
    pub fn disable() -> Result<()> {
        let mut hkey = HKEY::default();
        unsafe {
            let open_res = RegOpenKeyExW(
                HKEY_CURRENT_USER,
                RUN_SUBKEY,
                None,
                KEY_WRITE,
                &mut hkey,
            );

            if open_res != ERROR_SUCCESS {
                return Err(anyhow!("Failed to open Run registry key for writing: {:?}", open_res));
            }

            let del_res = RegDeleteValueW(hkey, APP_REG_NAME);
            let _ = RegCloseKey(hkey);

            if del_res != ERROR_SUCCESS && del_res != ERROR_FILE_NOT_FOUND {
                return Err(anyhow!("Failed to delete Run registry value: {:?}", del_res));
            }
        }

        tracing::info!("Auto-startup unregistered successfully");
        Ok(())
    }
}
