use ferinth::structures::project::Project;
use serde::{Deserialize, Serialize};

use super::structures::mod_source::ModSource;

#[derive(Serialize, Deserialize, Debug, Clone)]
// 序列化为 Json 后其值对应的就是名称
pub enum TargetKind {
    Local,
    Server,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Target {
    pub kind: TargetKind,
    pub location: String,
    pub mod_sources: Vec<ModSource>
}

impl Target {
    pub fn add_mod_source(&mut self, mod_source: ModSource) {
        self.mod_sources.push(mod_source);
    }
    
    pub async fn get_mod_sources_info(&self) -> Vec<Project> {
        let mut mod_sources_info = Vec::new();
        for mod_source in &self.mod_sources {
            if let Ok(mod_source_info) = mod_source.get_mod_source_info().await {
                mod_sources_info.push(mod_source_info);
            }
        }
        mod_sources_info
    }

    pub async fn update_mod_sources_info(&self) -> Vec<Project> {
        let mut mod_sources_info = Vec::new();
        for mod_source in &self.mod_sources {
            if let Ok(mod_source_info) = mod_source.update_mod_source_info().await {
                mod_sources_info.push(mod_source_info);
            }
        }
        mod_sources_info
    }
}