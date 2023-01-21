use crate::{amcm::{AMCM_DIR, cache::CACHE}, utils::file};

use ferinth::structures::version::Version;
use std::collections::HashMap;
use tokio::task;
use std::fs;
use std::io::Write;

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

#[tauri::command]
pub async fn get_version(id: String) -> Result<Version, String> {
    task::block_in_place(|| match CACHE.blocking_lock().get_version(&id) {
        Ok(version) => Ok(version),
        Err(err) => Err(format!("{}", err)),
    })
}

#[tauri::command]
pub async fn get_versions(ids: Vec<String>) -> Result<Vec<Version>, String> {
    task::block_in_place(|| match CACHE.blocking_lock().get_versions(ids) {
        Ok(versions) => Ok(versions),
        Err(err) => Err(format!("{}", err)),
    })
}

#[tauri::command]
pub async fn is_version_downloaded(id: String) -> Result<bool, String> {
    task::block_in_place(move || {
        match CACHE.blocking_lock().get_version(id.as_str()) {
            Ok(version) => {
                let path = AMCM_DIR
                    .join("storage")
                    .join("mods")
                    .join(version.project_id)
                    .join(format!("{}.jar", version.id));
                Ok(file::is_path_exist(path))
            }
            Err(err) => Err(format!("{}", err)),
        }
    })
}

#[tauri::command]
pub async fn download_version(id: String) -> Result<(), String>{
    task::block_in_place(|| match CACHE.blocking_lock().get_version(&id) {
        Ok(version) => {
            let client = reqwest::blocking::Client::new();
            println!("{:#?}", version.files);
            let url = version.files.first().unwrap().url.as_str();
            match client.get(url).send() {
                Ok(res) => {
                    match res.bytes() {
                        Ok(bytes) => {
                            let path = AMCM_DIR
                            .join("storage")
                            .join("mods")
                            .join(version.project_id)
                            .join(format!("{}.jar", version.id));
                            match fs::write(path, &bytes) {
                                Ok(_) => Ok(()),
                                Err(err) => Err(format!("{}", err)),
                            }
                        },
                        Err(err) => Err(format!("{}", err))
                    }
                },
                Err(err) => Err(format!("{}", err))
            }
        },
        Err(err) => Err(format!("{}", err)),
    })
}
