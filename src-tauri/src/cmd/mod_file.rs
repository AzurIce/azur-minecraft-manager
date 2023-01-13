use crate::amcm::{core::CORE, data::DATA};

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
pub async fn remove_local_mod_file(hash: String) -> Result<(), String> {
    match DATA.lock().await.remove_local_mod_file_from_hash(hash) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}