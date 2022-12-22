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
#[derive(Serialize, Deserialize, Debug)]
pub enum BelongState {
    Unknown,
    Modrinth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModFile {
    filename: String,
    sha1: String,
    belong_state: BelongState,
}

impl ModFile {
    pub fn default() -> ModFile {
        ModFile {
            filename: String::from(""),
            sha1: String::from(""),
            belong_state: BelongState::Unknown,
        }
    }

    pub async fn of(path: &PathBuf) -> ModFile {
        let filename = String::from(path.file_name().unwrap().to_str().unwrap());
        let sha1 = get_file_sha1(path.to_str().unwrap());
        let mut belong_state = BelongState::Unknown;

        if let Some(_) = CORE.lock().await.data().get_project_id_from_hash(&sha1) {
            belong_state = BelongState::Modrinth;
        }

        ModFile {
            filename,
            sha1,
            belong_state,
        }
    }
}