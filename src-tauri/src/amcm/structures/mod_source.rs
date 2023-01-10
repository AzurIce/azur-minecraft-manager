
use crate::{utils::file, amcm::{AMCM_DIR, cache::CACHE}};
use ferinth::{Ferinth, structures::project::Project};
use serde::{Deserialize, Serialize};
use futures::executor;

use std::{path::PathBuf, error::Error};
lazy_static! {
    static ref MOD_SOURCES_DIR: PathBuf = AMCM_DIR.clone().join("mods");
}

// ModSource //
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModSource {
	Modrinth(ModrinthModSource),
	// CurseForge,
	// Github,
	// ...
}

impl ModSource {
    pub async fn get_mod_source_info(&self) -> Result<Project, Box<dyn Error>> {
        match self {
            ModSource::Modrinth(source) => source.get_mod_source_info().await
        }
    }

    pub async fn update_mod_source_info(&self) -> Result<Project, Box<dyn Error>> {
        match self {
            ModSource::Modrinth(source) => source.update_mod_source_info().await
        }
    }
}

// ModrinthModSource //
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModrinthModSource {
    project_id: String,
}

impl ModrinthModSource {
    pub fn new(project_id: String) -> ModrinthModSource {
        ModrinthModSource { project_id }
    }

    pub fn mod_source_dir(&self) -> PathBuf {
        MOD_SOURCES_DIR.join(&self.project_id)
    }

    pub async fn get_mod_source_info(&self) -> Result<Project, Box<dyn Error>> {
        CACHE.lock().await.get_project(&self.project_id).await
    }

    pub async fn update_mod_source_info(&self) -> Result<Project, Box<dyn Error>> {
        CACHE.lock().await.update_project(&self.project_id).await
    }
}
