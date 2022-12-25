use crate::utils::file::get_file_sha1;
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
    pub filename: String,
    pub enabled: bool,
    pub sha1: String,
    pub belong_state: BelongState,
}

impl ModFile {
    pub fn default() -> ModFile {
        ModFile {
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
            filename,
            sha1,
            enabled,
            belong_state,
        }
    }
}
