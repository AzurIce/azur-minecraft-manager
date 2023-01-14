use crate::amcm::cache::CACHE;

use ferinth::structures::version::Version;
use std::collections::HashMap;

#[tauri::command]
pub async fn get_version_from_hash(hash: String) -> Result<Version, String> {
    let cache = CACHE.lock().await;
    println!("\n-> amcm/cmd/mcmod.rs/get_version_from_hash");
    let time_start = std::time::Instant::now();
    let res = match cache.get_version_from_hash(&hash) {
        Ok(version) => Ok(version),
        Err(err) => Err(format!("{}", err)),
    };
    println!("   total cost: {:#?}", time_start.elapsed());
    println!("<- amcm/cmd/mcmod.rs/get_version_from_hash\n");
    res
}

#[tauri::command]
pub async fn get_versions_from_hashes(hashes: Vec<String>) -> HashMap<String, Version> {
    CACHE.lock().await.get_versions_from_hashes(hashes)
}