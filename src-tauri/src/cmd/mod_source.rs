use super::target::get_target;
use crate::amcm::config::CONFIG;
use crate::amcm::structures::mod_source::{ModSource, ModrinthModSource};
use crate::amcm::core::CORE;

#[tauri::command]
pub async fn add_mod_source_to_target(target_id: usize, project_id: String) -> Result<(), String> {
    CONFIG.lock().await.add_mod_source_to_target(target_id, project_id)
}