use std::fs;
use std::path::{Path, PathBuf};
use crate::{mcmod::ModFile, CORE};
use ferinth::structures::{project_structs::Project, version_structs::Version};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sha1File {
    filename: String,
    sha1: String,
}

impl Sha1File {
    pub fn default() -> Sha1File {
        Sha1File {
            filename: String::from(""),
            sha1: String::from(""),
        }
    }

    pub fn of(path: &PathBuf) -> Sha1File {
        use crate::file::get_file_sha1;
        Sha1File {
            filename: String::from(path.file_name().unwrap().to_str().unwrap()),
            sha1: get_file_sha1(path.to_str().unwrap()),
            ..Sha1File::default()
        }
    }
}

#[tauri::command]
pub fn get_mod_file_list(path: String) -> Vec<Sha1File> {
    let mut mod_file_list: Vec<Sha1File> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let file_path = entry.unwrap().path();
        if file_path.is_file() && file_path.extension().unwrap() == "jar" {
            mod_file_list.push(Sha1File::of(&file_path));
        }
    }
    mod_file_list
}

#[tauri::command]
pub async fn update_data_from_hashes(hashes: Vec<String>) -> Result<(), ferinth::Error> {
    CORE.lock().unwrap().data().update_data_from_hashes(hashes).await
}

#[tauri::command]
pub async fn update_data_from_hash(hash: String) -> Result<(), ferinth::Error> {
    CORE.lock().unwrap().data().update_data_from_hash(hash).await
}

#[tauri::command]
pub fn get_version_from_hash(hash: String) -> Option<Version> {
    if let Some(version) = CORE.lock().unwrap().data().get_version_from_hash(hash) {
        Some(version.clone())
    } else {
        None
    }
}

#[tauri::command]
pub fn get_project_from_hash(hash: String) -> Option<Project> {
    if let Some(project) = CORE.lock().unwrap().data().get_project_from_hash(hash) {
        Some(project.clone())
    } else {
        None
    }
}

// #[tauri::command]
// pub async fn get_belonged_mod_file(path: String) -> Result<ModFile, String> {
//     match mcmod::get_belonged_mod_file(path.into()).await {
//         Ok(mod_file) => Ok(mod_file),
//         Err(error) => Err(error.to_string())
//     }
// }