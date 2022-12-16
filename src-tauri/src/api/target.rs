use crate::config;

#[tauri::command]
pub fn get_target_list_json() -> String {
    config::get_config().get_target_list_json()
}

#[tauri::command]
pub fn get_target_json(id: usize) -> String {
    config::get_config().get_target_json(id)
}

#[tauri::command]
pub fn add_target(target_json: String) {
    let target = config::Target::from_str(target_json);

    let mut config = config::get_config();
    config.add_target(target);
    config::save_config(config);
}