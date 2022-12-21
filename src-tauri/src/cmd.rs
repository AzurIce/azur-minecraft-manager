pub mod target;
pub mod mcmod;

use crate::CORE;

#[tauri::command]
pub async fn load() {
    CORE.lock().await.load();
}