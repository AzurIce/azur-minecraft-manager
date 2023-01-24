// use crate::amcm::data::DATA;
use crate::amcm::core::CORE;
use crate::amcm::structures::mod_file::{self, ModFile};

use tauri::Window;
use tokio::task;
use std::fs;
use std::path::Path;


// #[tauri::command]
// pub async fn delete_mod_file(hash: String) -> Result<(), String> {
//     task::block_in_place(|| {
//         let mod_file = DATA.blocking_lock().get_mod_file_from_hash(hash).unwrap();
//         match fs::remove_file(mod_file.path) {
//             Ok(_) => Ok(()),
//             Err(err) => Err(format!("{}", err))
//         }
//     })
// }

#[tauri::command]
pub async fn update_mod_files(dir: String) -> Vec<ModFile> {
    let dir = Path::new(dir.as_str()).join(".minecraft").join("mods");
    println!("\n-> amcm/data.rs/update_mod_files: {:#?}", dir);
    
    let mut mod_files: Vec<ModFile> = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let file_path = entry.unwrap().path();

        if file_path.is_file()
            && (file_path.extension().unwrap() == "jar"
                || file_path.extension().unwrap() == "disabled")
        {
            mod_files.push(ModFile::of(&file_path));
        }
    }
    println!("<- amcm/data.rs/update_mod_files\n");
    mod_files
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
