use std::fs;
use ferinth::Error;
use crate::modrinth;
use crate::modloader;
use crate::mcmod::{self, ModFile};

#[tauri::command]
pub fn get_mod_file_list(path: String) -> Vec<ModFile> {
    let mut mod_file_list: Vec<ModFile> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let file_path = entry.unwrap().path();
        if file_path.is_file() && file_path.extension().unwrap() == "jar" {
            mod_file_list.push(ModFile::of(file_path));
        }
    }
    mod_file_list
}

#[tauri::command]
pub fn get_mod_filename_list(path: String) -> Vec<String> {
    let mut mod_filename_list: Vec<String> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let file_path = entry.unwrap().path();
        if file_path.is_file() && file_path.extension().unwrap() == "jar" {
            mod_filename_list.push(String::from(file_path.file_name().unwrap().to_str().unwrap()));
        }
    }
    mod_filename_list
}

#[tauri::command]
pub async fn get_belonged_mod_file(path: String) -> Result<ModFile, String> {
    match mcmod::get_belonged_mod_file(path.into()).await {
        Ok(mod_file) => Ok(mod_file),
        Err(error) => Err(error.to_string())
    }
}