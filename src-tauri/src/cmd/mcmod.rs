use crate::amcm::data::DATA;
use crate::amcm::structures::mod_file::ModFile;
use crate::amcm::core::CORE;
use crate::amcm::config::CONFIG;
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
    DATA.lock().await.local_mod_files()
}

#[tauri::command]
pub async fn get_version_from_hash(hash: String) -> Result<Version, String> {
    let cache = CACHE.lock().await;
    println!("\n-> amcm/cmd/mcmod.rs/get_version_from_hash");
    let time_start = std::time::Instant::now();
    let res = match cache.get_version_from_hash(&hash).await {
        Ok(version) => Ok(version),
        Err(err) => Err(format!("{}", err)),
    };
    println!("   total cost: {:#?}", time_start.elapsed());
    println!("<- amcm/cmd/mcmod.rs/get_version_from_hash\n");
    res
}

#[tauri::command]
pub async fn get_versions_from_hashes(hashes: Vec<String>) -> HashMap<String, Version> {
    CACHE.lock().await.get_versions_from_hashes(hashes).await
}

#[tauri::command]
pub async fn update_local_mod_files(dir: String) -> Vec<ModFile> {
    DATA.lock().await.update_local_mod_files(dir).await
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
    let target = CONFIG.lock().await.get_target(index)?;
    println!("   get target cost: {:#?}", time_start1.elapsed());

    let time_start1 = std::time::Instant::now();
    let dir = Path::new(&target.location).join("mods/");
    let res = amcm.watch_mod_files(String::from(dir.to_str().unwrap()), window).await;
    println!("   watch mod files cost: {:#?}", time_start1.elapsed());

    println!("   total cost: {:#?}", time_start.elapsed());
    println!("<- amcm/cmd/mcmod.rs/target_watch_mod_files\n");
    res
}

