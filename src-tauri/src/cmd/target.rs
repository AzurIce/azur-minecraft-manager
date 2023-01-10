use crate::amcm::{core::CORE, target::Target};

#[tauri::command]
pub async fn get_target_list() -> Vec<Target> {
    CORE.lock().await.config().targets()
}

#[tauri::command]
pub async fn get_target(index: usize) -> Result<Target, String>{
    CORE.lock().await.config().get_target(index)
}


#[tauri::command]
pub async fn add_target(target: Target) -> Vec<Target> {
    println!("add_target: {:#?}", target);

    CORE.lock().await.add_target(target);
    get_target_list().await
}

#[tauri::command]
pub async fn del_target(index: usize) -> Vec<Target> {
    println!("del_target: {:#?}", index);

    CORE.lock().await.del_target(index);
    get_target_list().await
}