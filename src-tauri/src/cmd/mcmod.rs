use crate::amcm::structures::mod_file::ModFile;
use crate::amcm::core::CORE;
use crate::amcm::cache::CACHE;
use ferinth::structures::{project::Project, version::Version};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use tauri::Window;
use futures::executor;

#[tauri::command]
pub async fn get_local_mod_files() -> Vec<ModFile> {
    CORE.lock().await.data().local_mod_files()
}

// #[tauri::command]
// pub async fn get_mod_file_list(path: String) -> Vec<ModFile> {
//     let mut mod_file_list: Vec<ModFile> = Vec::new();
//     for entry in fs::read_dir(path).unwrap() {
//         let file_path = entry.unwrap().path();
//         if file_path.is_file() && file_path.extension().unwrap() == "jar" {
//             mod_file_list.push(ModFile::of(&file_path).await);
//         }
//     }
//     mod_file_list
// }

// #[tauri::command]
// pub async fn update_data_from_hashes(hashes: Vec<String>) -> Result<(), String> {
//     CORE.lock().await.update_data_from_hashes(hashes).await
// }

// #[tauri::command]
// pub async fn update_data_from_hash(hash: String) -> Result<(), String> {
//     CORE.lock().await.update_data_from_hash(hash).await
// }

// #[tauri::command]
// pub async fn get_versions_from_hashes(hashes: Vec<String>) -> Result<HashMap<String, Version>, String> {
//     let modrinth = ferinth::Ferinth::default();
//         let res = modrinth.get_versions_from_hashes(hashes).await;
//         if let Err(error) = res {
//             return Err(error.to_string());
//         }
//         Ok(res.unwrap())

// }

// #[tauri::command]
// pub async fn get_project_from_hash(hash: String) -> Option<Project> {
//     if let Some(project) = CORE.lock().await.data().get_project_from_hash(&hash) {
//         Some(project.clone())
//     } else {
//         None
//     }
// }

#[tauri::command]
pub async fn get_version_from_hash(hash: String) -> Result<Version, String> {
    println!("\n-> amcm/cmd/mcmod.rs/get_version_from_hash");
    let time_start = std::time::Instant::now();
    let res = match CACHE.lock().await.get_version_from_hash(&hash).await {
        Ok(version) => Ok(version),
        Err(err) => Err(format!("{}", err)),
    };
    println!("   total cost: {:#?}", time_start.elapsed());
    println!("<- amcm/cmd/mcmod.rs/get_version_from_hash\n");
    res
}

#[tauri::command]
pub async fn get_versions_from_hashes(hashes: Vec<String>) -> HashMap<String, Version> {
    println!("\n-> amcm/cmd/mcmod.rs/get_versions_from_hashes");
    let time_start = std::time::Instant::now();
    let res = CACHE.lock().await.get_versions_from_hashes(hashes).await;
    println!("   total cost: {:#?}", time_start.elapsed());
    println!("<- amcm/cmd/mcmod.rs/get_versions_from_hashes\n");
    res
}

// #[tauri::command]
// pub async fn get_project_from_hashes(hashes: Vec<String>) -> HashMap<String, Project> {
//     CORE.lock().await.data().get_project_from_hashes(hashes)
// }

#[tauri::command]
pub async fn update_local_mod_files(dir: String) -> Vec<ModFile> {
    CORE.lock().await.data().update_local_mod_files(dir).await
}

#[tauri::command]
pub async fn watch_mod_files(dir: String, window: Window) -> Result<(), String> {
    CORE.lock().await.watch_mod_files(dir, window).await
}

#[tauri::command]
pub async fn target_watch_mod_files(index: usize, window: Window) -> Result<(), String> {
    println!("\n-> amcm/cmd/mcmod.rs/target_watch_mod_files");
    let time_start = std::time::Instant::now();

    let mut amcm = CORE.lock().await;
    println!("   get lock cost: {:#?}", time_start.elapsed());

    let time_start1 = std::time::Instant::now();
    let target = amcm.config().get_target(index)?;
    println!("   get target cost: {:#?}", time_start1.elapsed());

    let time_start1 = std::time::Instant::now();
    let dir = Path::new(&target.location).join("mods/");
    let res = amcm.watch_mod_files(String::from(dir.to_str().unwrap()), window).await;
    println!("   watch mod files cost: {:#?}", time_start1.elapsed());

    println!("   total cost: {:#?}", time_start.elapsed());
    println!("<- amcm/cmd/mcmod.rs/target_watch_mod_files\n");
    res
}

#[tauri::command]
pub async fn set_mod_file_enabled(hash: String, enabled: bool) -> Result<(), String> {
    if let Some(mod_file) = CORE.lock().await.data().get_mod_file_from_hash(hash) {
        mod_file.set_enabled(enabled)
    } else {
        Err(String::from("Not found"))
    }
}