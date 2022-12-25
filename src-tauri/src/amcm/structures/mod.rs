use crate::utils::file::{get_file_sha1, self};
use crate::CORE;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

//----- ModFileBelong -----//
#[derive(Serialize, Deserialize, Debug)]
pub struct ModFileBelong {
    pub version_id: String,
    pub project_id: String,
}

impl ModFileBelong {
    pub fn new(version_id: String, project_id: String) -> ModFileBelong {
        ModFileBelong {
            version_id,
            project_id,
        }
    }
}

//----- ModFile -----//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BelongState {
    Unknown,
    Modrinth,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModFile {
    pub path: String,
    pub filename: String,
    pub enabled: bool,
    pub sha1: String,
    pub belong_state: BelongState,
}

impl ModFile {
    pub fn default() -> ModFile {
        ModFile {
            path: String::from(""),
            filename: String::from(""),
            sha1: String::from(""),
            enabled: true,
            belong_state: BelongState::Unknown,
        }
    }

    pub async fn of(path: &PathBuf) -> ModFile {
        let filename = String::from(path.file_name().unwrap().to_str().unwrap());
        let sha1 = get_file_sha1(path.to_str().unwrap());
        let enabled = path.extension().unwrap() == "jar";
        let mut belong_state = BelongState::Unknown;

        if let Some(_) = CORE.lock().await.data().get_project_id_from_hash(&sha1) {
            belong_state = BelongState::Modrinth;
        }

        ModFile {
            path: String::from(path.to_str().unwrap()),
            filename,
            sha1,
            enabled,
            belong_state,
        }
    }

    pub fn set_enabled(&self, enabled: bool) -> Result<(), String>{
        use std::fs;
        use std::time::Instant;
        let src = self.path.clone();
        let dst;
        if !(self.enabled ^ enabled) {
            return Ok(())
        }
        if self.enabled {
            dst = String::from(self.path.clone() + ".disabled");
        } else {
            dst = String::from(&self.path[..&self.path.len()-9]);
            println!("{:#?}", dst);
        }
        let time_start = Instant::now();
        if let Err(error) = fs::rename(src, dst) {
            Err(error.to_string())
        } else {
            let time_cost = time_start.elapsed();
            println!("Rename cost: {:#?}", time_cost);
            Ok(())
        }
    }
}
