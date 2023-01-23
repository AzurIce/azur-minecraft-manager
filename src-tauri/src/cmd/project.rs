use crate::amcm::cache::CACHE;

use ferinth::structures::project::Project;
use std::error::Error;
use tokio::task;

#[tauri::command]
pub async fn get_project(id: String) -> Result<Project, String> {
    task::block_in_place(|| match CACHE.blocking_lock().get_project(&id) {
        Ok(project) => Ok(project),
        Err(err) => Err(format!("{}", err)),
    })
}

#[tauri::command]
pub async fn update_project(id: String) -> Result<Project, String> {
    task::block_in_place(|| match CACHE.blocking_lock().update_project(&id) {
        Ok(project) => Ok(project),
        Err(err) => Err(format!("{}", err)),
    })
}
