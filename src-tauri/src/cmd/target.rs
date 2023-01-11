use crate::amcm::{core::CORE, target::Target, config::CONFIG};

#[tauri::command]
pub async fn get_target_list() -> Vec<Target> {
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
    get_target_list().await
}

#[tauri::command]
pub async fn del_target(index: usize) -> Vec<Target> {
    println!("del_target: {:#?}", index);

    CONFIG.lock().await.del_target(index);
    get_target_list().await
}