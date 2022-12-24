use crate::amcm::structures::ModFile;
use crate::utils::file::get_file_sha1;
use crate::CORE;
use ferinth::structures::{project_structs::Project, version_structs::Version};
use notify::recommended_watcher;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::Window;

#[tauri::command]
pub async fn get_mod_files() -> Vec<ModFile> {
    CORE.lock().await.data().mod_files()
}

#[tauri::command]
pub async fn get_mod_file_list(path: String) -> Vec<ModFile> {
    let mut mod_file_list: Vec<ModFile> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let file_path = entry.unwrap().path();
        if file_path.is_file() && file_path.extension().unwrap() == "jar" {
            mod_file_list.push(ModFile::of(&file_path).await);
        }
    }
    mod_file_list
}

#[tauri::command]
pub async fn update_data_from_hashes(hashes: Vec<String>, window: Window) -> Result<(), String> {
    let res = CORE.lock().await.update_data_from_hashes(hashes).await;
    window.emit("data_updated", "");
    res
}

#[tauri::command]
pub async fn update_data_from_hash(hash: String) -> Result<(), String> {
    CORE.lock().await.update_data_from_hash(hash).await
}

#[tauri::command]
pub async fn get_version_from_hash(hash: String) -> Option<Version> {
    if let Some(version) = CORE.lock().await.data().get_version_from_hash(&hash) {
        Some(version.clone())
    } else {
        None
    }
}

#[tauri::command]
pub async fn get_project_from_hash(hash: String) -> Option<Project> {
    if let Some(project) = CORE.lock().await.data().get_project_from_hash(&hash) {
        Some(project.clone())
    } else {
        None
    }
}

#[tauri::command]
pub async fn get_version_from_hashes(hashes: Vec<String>) -> HashMap<String, Version> {
    CORE.lock().await.data().get_version_from_hashes(hashes)
}

#[tauri::command]
pub async fn get_project_from_hashes(hashes: Vec<String>) -> HashMap<String, Project> {
    CORE.lock().await.data().get_project_from_hashes(hashes)
}

#[tauri::command]
pub async fn update_mod_files(dir: String) {
    CORE.lock().await.data().update_mod_files(dir).await;
}

#[tauri::command]
pub async fn watch_mod_files(dir: String, window: Window) -> Result<(), String> {
    CORE.lock().await.watch_mod_files(dir, window).await
}