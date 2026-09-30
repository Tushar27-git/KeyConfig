use anyhow::{Context, Result};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub struct ShortcutManager;

impl ShortcutManager {
    /// Returns the path to the user's Start Menu Programs folder
    pub fn get_start_menu_programs_dir() -> Result<PathBuf> {
        let appdata = std::env::var("APPDATA")
            .context("APPDATA environment variable not found")?;
        Ok(PathBuf::from(appdata).join(r"Microsoft\Windows\Start Menu\Programs"))
    }

    /// Checks if Theasus shortcut exists in the Start Menu
    pub fn is_installed() -> bool {
        if let Ok(dir) = Self::get_start_menu_programs_dir() {
            let primary = dir.join("Theasus.lnk");
            let alias = dir.join("Thesus.lnk");
            primary.is_file() || alias.is_file()
        } else {
            false
        }
    }

    /// Install/Repair the Start Menu shortcuts and Windows App Paths registry entries
    pub fn install(target_exe: &Path, working_dir: &Path, icon_path: Option<&Path>) -> Result<()> {
        let programs_dir = Self::get_start_menu_programs_dir()?;
        if !programs_dir.exists() {
            let _ = std::fs::create_dir_all(&programs_dir);
        }

        let exe_str = target_exe.to_string_lossy().replace('\'', "''");
        let work_str = working_dir.to_string_lossy().replace('\'', "''");
        let icon_str = icon_path
            .map(|p| p.to_string_lossy().replace('\'', "''"))
            .unwrap_or_else(|| exe_str.clone());

        let ps_script = format!(
            r#"$ws = New-Object -ComObject WScript.Shell;
$p = [Environment]::GetFolderPath('Programs');
$t1 = [System.IO.Path]::Combine($p, 'Theasus.lnk');
$s1 = $ws.CreateShortcut($t1);
$s1.TargetPath = '{exe}';
$s1.WorkingDirectory = '{work}';
$s1.Description = 'Theasus Keyboard Control Center (Native Remapper)';
$s1.IconLocation = '{icon},0';
$s1.Save();

$t2 = [System.IO.Path]::Combine($p, 'Thesus.lnk');
$s2 = $ws.CreateShortcut($t2);
$s2.TargetPath = '{exe}';
$s2.WorkingDirectory = '{work}';
$s2.Description = 'Thesus Keyboard Control Center';
$s2.IconLocation = '{icon},0';
$s2.Save();
"#,
            exe = exe_str,
            work = work_str,
            icon = icon_str
        );

        let output = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command")
            .arg(&ps_script)
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .context("Failed to run shortcut generation script")?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            tracing::warn!("PowerShell shortcut creation output: {}", err);
        }

        // Also register App Paths in Windows Registry for instant Win+R / Search bar resolution
        let _ = Self::register_app_paths("theasus.exe", target_exe, working_dir);
        let _ = Self::register_app_paths("thesus.exe", target_exe, working_dir);

        tracing::info!("Start Menu shortcuts installed successfully: Theasus.lnk & Thesus.lnk");
        Ok(())
    }

    /// Register executable in HKCU\Software\Microsoft\Windows\CurrentVersion\App Paths
    pub fn register_app_paths(alias_name: &str, target_exe: &Path, working_dir: &Path) -> Result<()> {
        let exe_str = target_exe.to_string_lossy();
        let work_str = working_dir.to_string_lossy();

        let key = format!(r"HKCU\Software\Microsoft\Windows\CurrentVersion\App Paths\{}", alias_name);

        let _ = Command::new("reg")
            .arg("add")
            .arg(&key)
            .arg("/ve")
            .arg("/t")
            .arg("REG_SZ")
            .arg("/d")
            .arg(&*exe_str)
            .arg("/f")
            .creation_flags(CREATE_NO_WINDOW)
            .output();

        let _ = Command::new("reg")
            .arg("add")
            .arg(&key)
            .arg("/v")
            .arg("Path")
            .arg("/t")
            .arg("REG_SZ")
            .arg("/d")
            .arg(&*work_str)
            .arg("/f")
            .creation_flags(CREATE_NO_WINDOW)
            .output();

        Ok(())
    }

    /// Remove the Start Menu shortcuts and App Paths
    pub fn uninstall() -> Result<()> {
        if let Ok(dir) = Self::get_start_menu_programs_dir() {
            let primary = dir.join("Theasus.lnk");
            let alias = dir.join("Thesus.lnk");
            if primary.exists() {
                let _ = std::fs::remove_file(primary);
            }
            if alias.exists() {
                let _ = std::fs::remove_file(alias);
            }
        }

        let _ = Command::new("reg")
            .arg("delete")
            .arg(r"HKCU\Software\Microsoft\Windows\CurrentVersion\App Paths\theasus.exe")
            .arg("/f")
            .creation_flags(CREATE_NO_WINDOW)
            .output();

        let _ = Command::new("reg")
            .arg("delete")
            .arg(r"HKCU\Software\Microsoft\Windows\CurrentVersion\App Paths\thesus.exe")
            .arg("/f")
            .creation_flags(CREATE_NO_WINDOW)
            .output();

        tracing::info!("Start Menu shortcuts and App Paths removed");
        Ok(())
    }
}
