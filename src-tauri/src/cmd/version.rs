use crate::{
    amcm::AMCM_DIR,
    utils::file,
};

use ferinth::structures::version::Version;
use ferinth::Error;
use ferinth::Ferinth;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::{collections::HashMap, time::Instant};
use tokio::task;

#[tauri::command]
pub async fn get_version_from_hash(hash: String) -> Result<Version, String> {
    println!("get_version_from_hash({})", hash);
    let modrinth = Ferinth::default();
    match modrinth.get_version_from_hash(&hash).await {
        Ok(version) => Ok(version),
        Err(err) => {
            // if let Error::RateLimitExceeded(s) = err {

            // }
            Err(format!("{}", err))
        }
    }
}

// #[tauri::command]
// pub async fn get_versions_from_hashes(hashes: Vec<String>) -> HashMap<String, Version> {
//     CACHE.lock().await.get_versions_from_hashes(hashes)
// }

#[tauri::command]
pub async fn get_version(id: String) -> Result<Version, String> {
    println!("get_version({})", id);
    let modrinth = Ferinth::default();
    match modrinth.get_version(&id).await {
        Ok(version) => Ok(version),
        Err(err) => Err(format!("{}", err)),
    }
}

#[tauri::command]
pub async fn get_versions(ids: Vec<String>) -> Result<Vec<Version>, String> {
    println!("get_versions({:#?})", ids);
    let modrinth = Ferinth::default();
    match modrinth
        .get_multiple_versions(ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>().as_slice())
        .await
    {
        Ok(versions) => Ok(versions),
        Err(err) => Err(format!("{}", err)),
    }
}

#[tauri::command]
pub async fn is_version_downloaded(target_dir: String, project_id: String, version_id: String) -> Result<bool, String> {
    let path = Path::new(&target_dir)
        .join(".amcm")
        .join("storage")
        .join(project_id)
        .join(format!("{}.jar", version_id));
    Ok(file::is_path_exist(path))
}

#[tauri::command]
pub async fn download_version(target_dir: String, project_id: String, version_id: String, file_url: String) -> Result<(), String> {

    let client = reqwest::blocking::Client::new();
    // println!("{:#?}", version.files);
    // let url = version.files.first().unwrap().url.as_str();
    match client.get(file_url).send() {
        Ok(res) => match res.bytes() {
            Ok(bytes) => {
                let path = Path::new(&target_dir)
                    .join(".amcm")
                    .join("storage")
                    .join(project_id)
                    .join(format!("{}.jar", version_id));
                fs::create_dir_all(path.parent().unwrap()).expect("cannot create dir");
                match fs::write(path, &bytes) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(format!("{}", err)),
                }
            }
            Err(err) => Err(format!("{}", err)),
        },
        Err(err) => Err(format!("{}", err)),
    }
}
