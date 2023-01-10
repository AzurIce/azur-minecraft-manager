pub mod target;
pub mod mcmod;

use crate::amcm::core::CORE;

#[tauri::command]
pub async fn load() {
    CORE.lock().await.load();
}