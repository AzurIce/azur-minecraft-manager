pub mod target;
pub mod mcmod;

use crate::CORE;

#[tauri::command]
pub fn load() {
    CORE.lock().unwrap().load();
}