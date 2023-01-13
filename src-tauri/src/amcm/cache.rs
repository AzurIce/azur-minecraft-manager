use std::{path::PathBuf, collections::HashMap, error::Error};

use ferinth::{structures::{project::Project, version::Version}, Ferinth};
use futures::executor;
use serde::{Deserialize, Serialize};

use crate::utils::file;

use super::AMCM_DIR;

use tokio::{sync::Mutex, runtime::Runtime};
lazy_static! {
    pub static ref CACHE: Mutex<Cache> = Mutex::new(Cache::default());
}
pub struct Cache {
    cache_dir: PathBuf,
    rt: Runtime,
}

impl Cache {
    pub fn default() -> Cache {
        Cache {
            cache_dir: AMCM_DIR.join("cache"),
            rt: Runtime::new().unwrap(),
        }
    }

    ///// project /////
    pub async fn get_project(&self, id: &str) -> Result<Project, Box<dyn Error>> {
        let path = self.cache_dir.join("projects").join(format!("{}.json", id));
        if !file::is_path_exist(path.as_path()) {
            return self.update_project(id).await;
        }

        let project_json = file::read_to_string(path)?;
        let project = serde_json::from_str(&project_json)?;
        Ok(project)
    }

    pub async fn update_project(&self, id: &str) -> Result<Project, Box<dyn Error>> {
        let modrinth = Ferinth::default();

        let path = self.cache_dir.join("projects").join(format!("{}.json", id));

        let project = modrinth.get_project(id).await?;
        let project_json = serde_json::to_string(&project)?;
        file::write_str(path, &project_json)?;
        
        Ok(project)
    }

    ///// version /////
    pub async fn get_version(&self, id: &str) -> Result<Version, Box<dyn Error>> {
        let path = self.cache_dir.join("versions").join(format!("{}.json", id));
        if !file::is_path_exist(path.as_path()) {
            return self.update_version(id).await;
        }

        let version_json = file::read_to_string(path)?;
        let version = serde_json::from_str(&version_json)?;
        Ok(version)
    }

    pub async fn update_version(&self, id: &str) -> Result<Version, Box<dyn Error>> {
        let modrinth = Ferinth::default();

        let path = self.cache_dir.join("versions").join(format!("{}.json", id));

        let version = modrinth.get_version(id).await?;
        let version_json = serde_json::to_string(&version)?;
        file::write_str(path, &version_json)?;
        
        Ok(version)
    }

    pub async fn get_versions(&self, ids: Vec<String>) -> Vec<Version> {
        let mut versions = Vec::new();
        for id in &ids {
            if let Ok(version) = self.get_version(id).await {
                versions.push(version);
            }
        }
        versions
    }

    pub async fn update_versions(&self, ids: Vec<String>) -> Vec<Version> {
        let mut versions = Vec::new();
        for id in &ids {
            if let Ok(version) = self.update_version(id).await {
                versions.push(version);
            }
        }
        versions
    }

    pub async fn get_version_from_hash(&self, hash: &str) -> Result<Version, Box<dyn Error>> {
        let path = self.cache_dir.join("sha1-version").join(hash);
        match file::is_path_exist(path.as_path()) {
            true => match file::read_to_string(path.as_path()) {
                Ok(id) => self.get_version(&id).await,
                Err(err) => self.update_version_from_hash(hash).await,
            },
            false => self.update_version_from_hash(hash).await,
        }
    }

    pub async fn update_version_from_hash(&self, hash: &str) -> Result<Version, Box<dyn Error>> {
        let modrinth = Ferinth::default();

        let version = modrinth.get_version_from_hash(hash).await?;

        let version_json = serde_json::to_string(&version)?;
        let path = self.cache_dir.join("versions").join(format!("{}.json", version.id));
        file::write_str(path, &version_json)?;

        let path = self.cache_dir.join("sha1-version").join(hash);
        file::write_str(path, &version.id)?;
        
        Ok(version)
    }

    pub async fn get_versions_from_hashes(&self, hashes: Vec<String>) -> HashMap<String, Version> {
        // println!("{:#?}", hashes);
        let mut sha1_to_version = HashMap::new();

        for hash in &hashes {
            if let Ok(version) = self.get_version_from_hash(hash).await {
                sha1_to_version.insert(hash.to_owned(), version);
            }
        }
        // println!("{:#?}", sha1_to_version);
        sha1_to_version
    }

    pub async fn update_versions_from_hashes(&self, hashes: Vec<String>) -> HashMap<String, Version> {
        let mut sha1_to_version = HashMap::new();

        for hash in &hashes {
            if let Ok(version) = self.update_version_from_hash(hash).await {
                sha1_to_version.insert(hash.to_owned(), version);
            }
        }
        sha1_to_version
    }
}