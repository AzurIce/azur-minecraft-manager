use std::{path::{Path, PathBuf}, error::Error, fs};

use serde::{Deserialize, Serialize};

use crate::{utils::file, amcm::cache::CACHE, amcm::AMCM_DIR};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct ModFile {
    pub path: String,
    pub filename: String,
    pub sha1: String,
    pub enabled: bool,
}

impl ModFile {
    pub fn default() -> ModFile {
        ModFile {
            path: String::from(""),
            filename: String::from(""),
            sha1: String::from(""),
            enabled: true,
        }
    }

    pub fn of<P: AsRef<Path>>(path: P) -> ModFile {
        let path = path.as_ref();
        let filename = String::from(path.file_name().unwrap().to_str().unwrap());
        let sha1 = file::get_file_sha1(path.to_str().unwrap());
        let enabled = path.extension().unwrap() == "jar";

        ModFile {
            path: String::from(path.to_str().unwrap()),
            filename,
            sha1,
            enabled,
        }
    }

    pub fn enable(&self) -> Result<(), String> {
        use std::time::Instant;
        if (self.enabled == true) {
            return Ok(());
        }

        let src = self.path.clone();
        let dst = String::from(&self.path[..&self.path.len() - 9]);

        let time_start = Instant::now();
        if let Err(error) = fs::rename(src, dst) {
            Err(error.to_string())
        } else {
            let time_cost = time_start.elapsed();
            println!("Rename cost: {:#?}", time_cost);
            Ok(())
        }
    }

    pub fn disable(&self) -> Result<(), String> {
        use std::fs;
        use std::time::Instant;

        if (self.enabled == false) {
            return Ok(());
        }

        let src = self.path.clone();
        let dst = String::from(self.path.clone() + ".disabled");

        let time_start = Instant::now();
        if let Err(error) = fs::rename(src, dst) {
            Err(error.to_string())
        } else {
            let time_cost = time_start.elapsed();
            println!("Rename cost: {:#?}", time_cost);
            Ok(())
        }
    }

    pub async fn move_to_storage(&self) -> Result<(), Box<dyn Error>> {
        let version = CACHE.lock().await.get_version_from_hash(&self.sha1).await?;
        let dst = AMCM_DIR
            .join("storage")
            .join("mods")
            .join(version.project_id)
            .join(format!("{}.jar", version.id));
        if let Some(dir) = dst.parent() {
            fs::create_dir_all(dir)?;
            fs::copy(&self.path, dst)?;
            let mod_file_path = Path::new(&self.path);
            fs::rename(&mod_file_path, mod_file_path.parent().unwrap().join(format!("_amcm_{}", self.filename)))?;
        }
        Ok(())
    }
}
