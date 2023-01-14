pub mod target;
pub mod mod_file;
pub mod mod_source;
pub mod version;
pub mod project;

#[cfg(test)]
pub mod mod_source_test;

// use crate::amcm::core::CORE;

// #[tauri::command]
// pub async fn load() {
//     CORE.lock().await.load();
// }