use crate::profiles::model::Profile;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct ProfileStore {
    dir: PathBuf,
}

impl ProfileStore {
    pub fn new() -> Self {
        let dir = if let Ok(appdata) = std::env::var("APPDATA") {
            let theasus_dir = PathBuf::from(&appdata).join("Theasus").join("profiles");
            let legacy_dir = PathBuf::from(&appdata).join("KeyboardControlCenter").join("profiles");
            if legacy_dir.exists() && !theasus_dir.exists() {
                legacy_dir
            } else {
                theasus_dir
            }
        } else {
            PathBuf::from("profiles")
        };

        let _ = fs::create_dir_all(&dir);
        Self { dir }
    }

    pub fn load_all(&self) -> HashMap<String, Profile> {
        let mut profiles = HashMap::new();

        let default_prof = Profile::new_default();
        let gaming_prof = Profile::new_gaming();
        let work_prof = Profile::new_work();

        profiles.insert(default_prof.id.clone(), default_prof);
        profiles.insert(gaming_prof.id.clone(), gaming_prof);
        profiles.insert(work_prof.id.clone(), work_prof);

        if let Ok(entries) = fs::read_dir(&self.dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(profile) = serde_json::from_str::<Profile>(&content) {
                            profiles.insert(profile.id.clone(), profile);
                        }
                    }
                }
            }
        }

        profiles
    }

    pub fn save_profile(&self, profile: &Profile) -> Result<()> {
        let file_path = self.dir.join(format!("{}.json", profile.id));
        let json = serde_json::to_string_pretty(profile)
            .context("Failed to serialize profile to JSON")?;

        let mut file = File::create(&file_path)
            .with_context(|| format!("Failed to create profile file {:?}", file_path))?;
        file.write_all(json.as_bytes())
            .context("Failed to write profile content")?;

        Ok(())
    }

    pub fn delete_profile(&self, id: &str) -> Result<()> {
        let file_path = self.dir.join(format!("{}.json", id));
        if file_path.exists() {
            fs::remove_file(&file_path)
                .with_context(|| format!("Failed to delete profile file {:?}", file_path))?;
        }
        Ok(())
    }

    pub fn get_storage_path(&self) -> &Path {
        &self.dir
    }
}
