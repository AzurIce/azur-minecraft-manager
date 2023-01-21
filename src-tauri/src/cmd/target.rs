use crate::amcm::{target::Target, config::CONFIG, cache::CACHE, AMCM_DIR};

use tokio::task;
use std::{path, fs};

#[tauri::command]
pub async fn get_targets() -> Vec<Target> {
    CONFIG.lock().await.targets()
}

#[tauri::command]
pub async fn get_target(index: usize) -> Result<Target, String>{
    CONFIG.lock().await.get_target(index)
}


#[tauri::command]
pub async fn add_target(target: Target) -> Vec<Target> {
    println!("add_target: {:#?}", target);

    CONFIG.lock().await.add_target(target);
    get_targets().await
}

#[tauri::command]
pub async fn del_target(index: usize) -> Vec<Target> {
    println!("del_target: {:#?}", index);

    CONFIG.lock().await.del_target(index);
    get_targets().await
}

#[tauri::command]
pub async fn copy_version_to_target(version_id: String, target_id: usize) -> Result<(), String>{
    task::block_in_place(|| {
        let target = CONFIG.blocking_lock().get_target(target_id).unwrap();
        let version = CACHE.blocking_lock().get_version(&version_id).unwrap();
        let src = AMCM_DIR
        .join("storage")
        .join("mods")
        .join(version.project_id)
        .join(format!("{}.jar", version.id));
        let dst = path::Path::new(&target.location).join("mods").join(format!("amcm_{}",version.files.first().unwrap().filename));
        match fs::copy(src, dst) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{}", err))
        }
    })
}