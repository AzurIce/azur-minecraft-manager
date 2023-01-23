use crate::{
    amcm::{cache::CACHE, AMCM_DIR},
    utils::file,
};

use ferinth::structures::version::Version;
use std::fs;
use std::path::Path;
use std::io::Write;
use std::{collections::HashMap, time::Instant};
use tokio::task;

#[tauri::command]
pub async fn get_version_from_hash(hash: String) -> Result<Version, String> {
    // println!("-> get_version_from_hash({})", hash);
    task::block_in_place(|| {
        let time_start = Instant::now();
        let res = match CACHE.blocking_lock().get_version_from_hash(&hash) {
            Ok(version) => Ok(version),
            Err(err) => Err(format!("{}", err)),
        };
        println!(
            "<- get_version_from_hash({}) cost {:#?}",
            hash,
            time_start.elapsed()
        );
        res
    })
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
pub async fn is_version_downloaded(target_dir: String, version_id: String) -> Result<bool, String> {
    task::block_in_place(
        move || match CACHE.blocking_lock().get_version(version_id.as_str()) {
            Ok(version) => {
                let path = Path::new(&target_dir).join(".amcm")
                    .join("storage")
                    .join(version.project_id)
                    .join(format!("{}.jar", version.id));
                Ok(file::is_path_exist(path))
            }
            Err(err) => Err(format!("{}", err)),
        },
    )
}

#[tauri::command]
pub async fn download_version(target_dir: String, id: String) -> Result<(), String> {
    task::block_in_place(|| match CACHE.blocking_lock().get_version(&id) {
        Ok(version) => {
            let client = reqwest::blocking::Client::new();
            println!("{:#?}", version.files);
            let url = version.files.first().unwrap().url.as_str();
            match client.get(url).send() {
                Ok(res) => match res.bytes() {
                    Ok(bytes) => {
                        let path = Path::new(&target_dir).join(".amcm")
                            .join("storage")
                            .join(version.project_id)
                            .join(format!("{}.jar", version.id));
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
        Err(err) => Err(format!("{}", err)),
    })
}
