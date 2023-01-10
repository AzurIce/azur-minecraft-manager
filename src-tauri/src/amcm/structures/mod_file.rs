use std::path::{PathBuf, Path};

use serde::{Serialize, Deserialize};

use crate::utils::file;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModFile {
    pub path: String,
    pub filename: String,
    pub sha1: String,
    pub enabled: bool,
    // pub belong_state: BelongState,
}

impl ModFile {
    pub fn default() -> ModFile {
        ModFile {
            path: String::from(""),
            filename: String::from(""),
            sha1: String::from(""),
            enabled: true,
            // belong_state: BelongState::Unknown,
        }
    }

    pub fn of<P: AsRef<Path>>(path: P) -> ModFile {
        let path = path.as_ref();
        let filename = String::from(path.file_name().unwrap().to_str().unwrap());
        let sha1 = file::get_file_sha1(path.to_str().unwrap());
        let enabled = path.extension().unwrap() == "jar";
        // let mut belong_state = BelongState::Unknown;

        // if let Some(_) = CORE.lock().await.data().get_project_id_from_hash(&sha1) {
        //     belong_state = BelongState::Modrinth;
        // }

        ModFile {
            path: String::from(path.to_str().unwrap()),
            filename,
            sha1,
            enabled,
            // belong_state,
        }
    }

    pub fn enable(&self) -> Result<(), String> {
        use std::fs;
        use std::time::Instant;
        if (self.enabled == true) {
            return Ok(())
        }

        let src = self.path.clone();
        let dst = String::from(&self.path[..&self.path.len()-9]);

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
            return Ok(())
        }

        let src = self.path.clone();
        let dst= String::from(self.path.clone() + ".disabled");

        let time_start = Instant::now();
        if let Err(error) = fs::rename(src, dst) {
            Err(error.to_string())
        } else {
            let time_cost = time_start.elapsed();
            println!("Rename cost: {:#?}", time_cost);
            Ok(())
        }
    }

    // pub fn set_enabled(&self, enabled: bool) -> Result<(), String>{
    //     use std::fs;
    //     use std::time::Instant;
    //     let src = self.path.clone();
    //     let dst;
    //     if !(self.enabled ^ enabled) {
    //         return Ok(())
    //     }
    //     if self.enabled {
    //         dst = String::from(self.path.clone() + ".disabled");
    //     } else {
    //         dst = String::from(&self.path[..&self.path.len()-9]);
    //         println!("{:#?}", dst);
    //     }
    //     let time_start = Instant::now();
    //     if let Err(error) = fs::rename(src, dst) {
    //         Err(error.to_string())
    //     } else {
    //         let time_cost = time_start.elapsed();
    //         println!("Rename cost: {:#?}", time_cost);
    //         Ok(())
    //     }
    // }
}