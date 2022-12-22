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
pub async fn update_data_from_hashes(hashes: Vec<String>) -> Result<(), String> {
    CORE.lock().await.update_data_from_hashes(hashes).await
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

// #[tauri::command]
// pub async fn enter_manage_target(index: usize, window: Window) -> Result<(), String> {
//     let mut amcm = CORE.lock().await;
//     let target = amcm.config().get_target(index)?;
//     println!("enter_manage_target: {:#?}", target);

//     amcm.update_mod_files(target.location.clone()).await;
//     window.emit("mod-files-updated", amcm.data().mod_files());
//     println!("emitted mod-files-updated: {:#?}", amcm.data().mod_files());

//     amcm.watch_target(target, window);
//     Ok(())
// }

// #[tauri::command]
// pub async fn leave_manage_target(index: usize) -> Result<(), String> {
//     let mut amcm = CORE.lock().await;
//     let target = amcm.config().get_target(index)?;
//     println!("leave_manage_target: {:#?}", target);

//     amcm.unwatch_target(target);
//     Ok(())
// }

#[tauri::command]
pub async fn update_mod_files(dir: String) {
    CORE.lock().await.data().update_mod_files(dir);
}

#[tauri::command]
pub async fn watch_mod_files(dir: String, window: Window) -> Result<(), String> {
    CORE.lock().await.watch_mod_files(dir, window).await
}

// #[tauri::command]
// pub async fn listen_mod_files(dir: String, window: Window) -> Result<String, String> {
//     use notify::RecursiveMode;
//     use notify_debouncer_mini::new_debouncer;
//     use std::{path::Path, time::Duration};

//     // setup debouncer
//     let (tx, rx) = std::sync::mpsc::channel();

//     // No specific tickrate, max debounce time 2 seconds
//     let debouncer = std::sync::Arc::new(std::sync::Mutex::new(
//         new_debouncer(Duration::from_secs(2), None, tx).unwrap(),
//     ));

//     debouncer
//         .watcher()
//         .watch(dir.clone(), RecursiveMode::NonRecursive)
//         .unwrap();

//     window.once("unlisten_mod_dir", move |_| {
//         debouncer.watcher.unwatch(dir.clone()).unwrap();
//     });

//     loop {
//         match rx.recv() {
//             Ok(RawEvent {
//                 path: Some(path),
//                 op: Ok(op),
//                 ..
//             }) => {
//                 //window.emit("changes", path.to_str().unwrap().to_string());
//                 use notify::op;

//                 let event = match op {
//                     op::CREATE => "create".to_string(),
//                     op::REMOVE => "remove".to_string(),
//                     op::RENAME => "rename".to_string(),
//                     _ => "unknown".to_string(),
//                 };

//                 if event != "unknown" {
//                     window
//                         .emit(
//                             "changes",
//                             Event {
//                                 path: path.to_str().unwrap().to_string(),
//                                 event,
//                             },
//                         )
//                         .unwrap();
//                 }
//             }
//             Ok(event) => eprintln!("broken event: {:?}", event),
//             Err(e) => break Err(e.to_string()),
//         }
//     }
// }

// #[tauri::command]
// pub async fn get_belonged_mod_file(path: String) -> Result<ModFile, String> {
//     match mcmod::get_belonged_mod_file(path.into()).await {
//         Ok(mod_file) => Ok(mod_file),
//         Err(error) => Err(error.to_string())
//     }
// }
