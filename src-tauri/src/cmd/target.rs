use crate::{CORE, config::Target};

// #[tauri::command]
// pub fn get_target_list_json() -> String {
//     CORE.lock().unwrap().config().get_target_list_json()
// }

#[tauri::command]
pub fn get_target_list() -> Vec<Target> {
    CORE.lock().unwrap().config().targets()
}

// #[tauri::command]
// pub fn get_target_json(id: usize) -> String {
//     CORE.lock().unwrap().config().get_target_json(id)
// }

#[tauri::command]
pub fn get_target(index: usize) -> Result<Target, String>{
    CORE.lock().unwrap().config().get_target(index)
}


#[tauri::command]
pub fn add_target(target: Target) -> Vec<Target> {
    // let target = serde_json::from_str(&target_json).unwrap();
    println!("add_target: {:#?}", target);

    // let mut config = CORE.lock().unwrap().config;
    CORE.lock().unwrap().add_target(target);
    // config::save_config(config);
    get_target_list()
}

#[tauri::command]
pub fn del_target(index: usize) -> Vec<Target> {
    println!("del_target: {:#?}", index);

    // let mut config = CORE.lock().unwrap().config;
    CORE.lock().unwrap().del_target(index);
    // config::save_config(config);
    get_target_list()
}