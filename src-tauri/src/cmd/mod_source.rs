use crate::amcm::config::CONFIG;
use crate::amcm::data::DATA;

#[tauri::command]
pub async fn add_mod_source(project_id: String) -> Result<(), String> {
    match CONFIG.lock().await.add_mod_source(project_id) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}

#[tauri::command]
pub async fn add_mod_source_from_local_mod_file(
    project_id: String,
    version_id: String,
    mod_file_hash: String,
) -> Result<(), String> {
    add_mod_source(project_id.clone()).await?;

    let mut data = DATA.lock().await;
    if let Some(mod_file) = data.get_mod_file_from_hash(mod_file_hash.clone()) {
        if let Ok(_) = mod_file.move_to_storage().await {
            if let Ok(_) = data.remove_mod_file_from_hash(mod_file_hash.clone()) {
                return Ok(());
            }
        }
    }
    return Err("err".to_owned());
}

#[tauri::command]
pub async fn add_mod_source_to_target(target_id: usize, project_id: String) -> Result<(), String> {
    match CONFIG
        .lock()
        .await
        .add_mod_source_to_target(target_id, project_id)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}
