use crate::amcm::data::DATA;
use crate::amcm::core::CORE;
use crate::amcm::structures::mod_file::ModFile;

use tauri::Window;

#[tauri::command]
pub async fn enable_mod_file(hash: String) -> Result<(), String> {
    if let Some(mod_file) = DATA.lock().await.get_mod_file_from_hash(hash) {
        mod_file.enable()
    } else {
        Err("Not found".to_owned())
    }
}

#[tauri::command]
pub async fn disable_mod_file(hash: String) -> Result<(), String> {
    if let Some(mod_file) = DATA.lock().await.get_mod_file_from_hash(hash) {
        mod_file.disable()
    } else {
        Err("Not found".to_owned())
    }
}

#[tauri::command]
pub async fn remove_mod_file(hash: String) -> Result<(), String> {
    match DATA.lock().await.remove_mod_file_from_hash(hash) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}

#[tauri::command]
pub async fn get_mod_files() -> Vec<ModFile> {
    DATA.lock().await.mod_files()
}

#[tauri::command]
pub async fn update_mod_files(dir: String) -> Vec<ModFile> {
    DATA.lock().await.update_mod_files(dir)
}

#[tauri::command]
pub async fn watch_mod_files(dir: String, window: Window) -> Result<(), String> {
    println!("\n-> amcm/cmd/mod_file.rs/watch_mod_files");
    let time_start = std::time::Instant::now();

    let res = CORE.lock().await.watch_mod_files(dir, window).await;

    println!("   total cost: {:#?}", time_start.elapsed());
    println!("<- amcm/cmd/mod_file.rs/watch_mod_files\n");
    res
}

// #[tauri::command]
// pub async fn target_watch_mod_files(index: usize, window: Window) -> Result<(), String> {
//     println!("\n-> amcm/cmd/mod_file.rs/target_watch_mod_files");
//     let time_start = std::time::Instant::now();

//     let mut amcm = CORE.lock().await;

//     let target = CONFIG.lock().await.get_target(index)?;
//     let dir = Path::new(&target.location).join("mods/");
//     let res = amcm.watch_mod_files(String::from(dir.to_str().unwrap()), window).await;

//     println!("   total cost: {:#?}", time_start.elapsed());
//     println!("<- amcm/cmd/mod_file.rs/target_watch_mod_files\n");
//     res
// }

