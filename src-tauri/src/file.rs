use std::fs;

#[tauri::command]
pub fn is_path_exist(path: &str) -> bool {
    return fs::try_exists(path).unwrap_or(false);
}