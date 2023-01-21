use crate::amcm::{target::Target, config::CONFIG};

#[tauri::command]
pub async fn get_targets() -> Vec<Target> {
    CONFIG.lock().await.targets()
}

#[tauri::command]
pub async fn get_target(index: usize) -> Result<Target, String>{
    CONFIG.lock().await.get_target(index)
}


#[tauri::command]
pub async fn add_target(target: Target) -> Vec<Target> {
    println!("add_target: {:#?}", target);

    CONFIG.lock().await.add_target(target);
    get_targets().await
}

#[tauri::command]
pub async fn del_target(index: usize) -> Vec<Target> {
    println!("del_target: {:#?}", index);

    CONFIG.lock().await.del_target(index);
    get_targets().await
}

#[tauri::command]
pub async fn choose_version_for_target(version_id: String, target_id: usize) -> Result<(), String>{
    // TODO:
    Ok(())
}