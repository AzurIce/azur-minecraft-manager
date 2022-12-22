use crate::amcm::structures::ModFileBelong;
use ferinth::structures::{project_structs::Project, version_structs::Version};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    versions: HashMap<String, Version>,
    projects: HashMap<String, Project>,
    hash_to_mfb: HashMap<String, ModFileBelong>,
}

impl Data {
    pub fn default() -> Data {
        Data {
            versions: HashMap::new(),
            projects: HashMap::new(),
            hash_to_mfb: HashMap::new(),
        }
    }

    pub fn get_version_id_from_hash(&self, hash: &String) -> Option<&String> {
        if let Some(mfb) = self.hash_to_mfb.get(hash) {
            Some(&mfb.version_id)
        } else {
            None
        }
    }
    pub fn get_project_id_from_hash(&self, hash: &String) -> Option<&String> {
        if let Some(mfb) = self.hash_to_mfb.get(hash) {
            Some(&mfb.project_id)
        } else {
            None
        }
    }
    pub fn get_version_from_hash(&self, hash: &String) -> Option<&Version> {
        if let Some(version_id) = self.get_version_id_from_hash(hash) {
            self.versions.get(version_id)
        } else {
            None
        }
    }
    pub fn get_project_from_hash(&self, hash: &String) -> Option<&Project> {
        if let Some(project_id) = self.get_project_id_from_hash(hash) {
            self.projects.get(project_id)
        } else {
            None
        }
    }

    pub fn get_version_from_hashes(&self, hashes: Vec<String>) -> HashMap<String, Version> {
        let mut versions = HashMap::new();
        for hash in hashes {
            if let Some(version) = self.get_version_from_hash(&hash) {
                versions.insert(hash, version.clone());
            }
        }
        versions
    }
    pub fn get_project_from_hashes(&self, hashes: Vec<String>) -> HashMap<String, Project> {
        let mut projects = HashMap::new();
        for hash in hashes {
            if let Some(project) = self.get_project_from_hash(&hash) {
                projects.insert(hash, project.clone());
            }
        }
        projects
    }

    pub async fn update_data_from_hashes(&mut self, hashes: Vec<String>) -> Result<(), String> {
        let mut project_ids: Vec<&str> = Vec::new();

        let modrinth = ferinth::Ferinth::default();
        let res = modrinth.get_versions_from_hashes(hashes).await;
        if let Err(error) = res {
            return Err(error.to_string());
        }
        let res = res.unwrap();

        for (hash, version) in &res {
            self.hash_to_mfb.insert(
                hash.clone(),
                ModFileBelong::new(version.id.clone(), version.project_id.clone()),
            );
            self.versions.insert(version.id.clone(), version.clone());
            project_ids.push(version.project_id.as_str());
        }

        match modrinth.get_multiple_projects(&project_ids[..]).await {
            Ok(res) => {
                for project in res {
                    let project_id = project.id.clone();
                    self.projects.insert(project_id, project);
                }
            }
            Err(error) => {
                return Err(error.to_string());
            }
        }

        Ok(())
    }
    pub async fn update_data_from_hash(&mut self, hash: String) -> Result<(), String> {
        let modrinth = ferinth::Ferinth::default();

        let version_id: String;
        let project_id: String;
        match modrinth.get_version_from_hash(&hash).await {
            Ok(version) => {
                version_id = version.id.clone();
                project_id = version.project_id.clone();
                self.hash_to_mfb.insert(
                    hash,
                    ModFileBelong::new(version_id.clone(), project_id.clone()),
                );
                self.versions.insert(version_id, version);
            }
            Err(error) => return Err(error.to_string()),
        }
        match modrinth.get_project(&project_id).await {
            Ok(project) => {
                self.projects.insert(project_id, project);
            }
            Err(error) => return Err(error.to_string()),
        }
        Ok(())
    }
}
