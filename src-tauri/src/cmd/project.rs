// use crate::amcm::cache::CACHE;

use ferinth::structures::project::Project;
use ferinth::Ferinth;
use std::error::Error;
use tokio::task;

#[tauri::command]
pub async fn get_project(id: String) -> Result<Project, String> {
    println!("get_project({})", id);
    let modrinth = Ferinth::default();
    match modrinth.get_project(&id).await {
        Ok(project) => Ok(project),
        Err(err) => Err(format!("{}", err)),
    }
}

#[tauri::command]
pub async fn get_projects(ids: Vec<String>) -> Result<Vec<Project>, String> {
    println!("get_projects({:#?})", ids);
    let modrinth = Ferinth::default();
    match modrinth
        .get_multiple_projects(
            ids.iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        )
        .await
    {
        Ok(projects) => Ok(projects),
        Err(err) => Err(format!("{}", err)),
    }
}
