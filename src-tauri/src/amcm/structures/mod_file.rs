use std::{error::Error, fs, path::Path};

use ferinth::structures::version::Version;
use serde::{Deserialize, Serialize};
use tokio::task;

use crate::{amcm::cache::CACHE, amcm::AMCM_DIR, utils::file};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModFile {
    pub path: String,
    pub filename: String,
    pub sha1: String,
    // pub remote_version: Option<Version>,
    pub enabled: bool,
}

impl ModFile {
    pub fn default() -> ModFile {
        ModFile {
            path: String::from(""),
            filename: String::from(""),
            sha1: String::from(""),
            // remote_version: None,
            enabled: true,
        }
    }

    pub fn of<P: AsRef<Path>>(path: P) -> ModFile {
        // task::block_in_place(|| {
            use std::time::Instant;
            let time_start = Instant::now();
            let path = path.as_ref();
            let filename = String::from(path.file_name().unwrap().to_str().unwrap());
            let sha1 = file::get_file_sha1(path.to_str().unwrap());
            let enabled = path.extension().unwrap() == "jar";
            println!(
                "       get file attributes cost: {:#?}",
                time_start.elapsed()
            );

            // let time_start1 = Instant::now();
            // let cache = CACHE.blocking_lock();
            // let remote_version = match cache.get_version_from_hash(&sha1) {
            //     Ok(version) => Some(version),
            //     Err(_) => None,
            // };
            // println!(
            //     "       get_remote_version cost: {:#?}",
            //     time_start1.elapsed()
            // );

            println!("   mod_file_of cost: {:#?}", time_start.elapsed());
            ModFile {
                path: String::from(path.to_str().unwrap()),
                filename,
                sha1,
                // remote_version,
                enabled,
            }
        // })
    }

    pub fn enable(&self) -> Result<(), String> {
        use std::time::Instant;

        if self.enabled {
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
        use std::time::Instant;

        if !self.enabled {
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
        let version = CACHE.lock().await.get_version_from_hash(&self.sha1)?;
        let dst = AMCM_DIR
            .join("storage")
            .join("mods")
            .join(version.project_id)
            .join(format!("{}.jar", version.id));
        if let Some(dir) = dst.parent() {
            fs::create_dir_all(dir)?;
            fs::copy(&self.path, dst)?;
            let mod_file_path = Path::new(&self.path);
            fs::rename(
                &mod_file_path,
                mod_file_path
                    .parent()
                    .unwrap()
                    .join(format!("_amcm_{}", self.filename)),
            )?;
        }
        Ok(())
    }
}
