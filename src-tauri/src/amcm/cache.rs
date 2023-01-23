use std::{collections::HashMap, error::Error, path::PathBuf};

use ferinth::{
    structures::{project::Project, version::Version},
    Ferinth,
};

use crate::utils::file;

use super::AMCM_DIR;

use tokio::runtime::Runtime;
use tokio::sync::Mutex;
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
    pub fn get_project(&self, id: &str) -> Result<Project, Box<dyn Error>> {
        let path = self.cache_dir.join("projects").join(format!("{}.json", id));
        if !file::is_path_exist(path.as_path()) {
            return self.update_project(id);
        }

        let project_json = file::read_to_string(path)?;
        let project = serde_json::from_str(&project_json)?;
        Ok(project)
    }

    pub fn update_project(&self, id: &str) -> Result<Project, Box<dyn Error>> {
        let modrinth = Ferinth::default();

        let path = self.cache_dir.join("projects").join(format!("{}.json", id));

        let project = self.rt.block_on(modrinth.get_project(id))?;
        let project_json = serde_json::to_string(&project)?;
        file::write_str(path, &project_json)?;

        Ok(project)
    }

    ///// version /////
    pub fn get_version(&self, id: &str) -> Result<Version, Box<dyn Error>> {
        let path = self.cache_dir.join("versions").join(format!("{}.json", id));
        if !file::is_path_exist(path.as_path()) {
            return self.update_version(id);
        }

        let version_json = file::read_to_string(path)?;
        let version = serde_json::from_str(&version_json)?;
        Ok(version)
    }
    pub fn get_cached_version(&self, id: &str) -> Result<Version, Box<dyn Error>> {
        let path = self.cache_dir.join("versions").join(format!("{}.json", id));
        let version_json = file::read_to_string(path)?;
        let version = serde_json::from_str(&version_json)?;
        Ok(version)
    }

    pub fn update_version(&self, id: &str) -> Result<Version, Box<dyn Error>> {
        let modrinth = Ferinth::default();

        let path = self.cache_dir.join("versions").join(format!("{}.json", id));

        let version = self.rt.block_on(modrinth.get_version(id))?;
        let version_json = serde_json::to_string(&version)?;
        file::write_str(path, &version_json)?;

        Ok(version)
    }

    pub fn get_versions(&self, ids: Vec<String>) -> Result<Vec<Version>, Box<dyn Error>> {
        let modrinth = Ferinth::default();
        let ids: Vec<&str> = ids.iter().map(|s| &s[..]).collect();

        let versions = self.rt.block_on(
            modrinth.get_multiple_versions(&ids),
        )?;
        for version in &versions {
            let path = self
                .cache_dir
                .join("versions")
                .join(format!("{}.json", version.id));
            let version_json = serde_json::to_string(&version)?;
            file::write_str(path, &version_json)?;
        }
        Ok(versions)
    }

    pub fn update_versions(&self, ids: Vec<String>) -> Vec<Version> {
        let mut versions = Vec::new();
        for id in &ids {
            if let Ok(version) = self.update_version(id) {
                versions.push(version);
            }
        }
        versions
    }

    pub fn get_cached_version_from_hash(&self, hash: &str) -> Result<Version, Box<dyn Error>> {
        let path = self.cache_dir.join("sha1-version").join(hash);
        let id = file::read_to_string(path.as_path())?;
        self.get_cached_version(&id)
    }
    pub fn get_version_from_hash(&self, hash: &str) -> Result<Version, Box<dyn Error>> {
        let path = self.cache_dir.join("sha1-version").join(hash);
        match file::is_path_exist(path.as_path()) {
            true => match file::read_to_string(path.as_path()) {
                Ok(id) => self.get_version(&id),
                Err(err) => self.update_version_from_hash(hash),
            },
            false => self.update_version_from_hash(hash),
        }
    }

    pub fn update_version_from_hash(&self, hash: &str) -> Result<Version, Box<dyn Error>> {
        let modrinth = Ferinth::default();

        let version = self.rt.block_on(modrinth.get_version_from_hash(hash))?;

        let version_json = serde_json::to_string(&version)?;
        let path = self
            .cache_dir
            .join("versions")
            .join(format!("{}.json", version.id));
        file::write_str(path, &version_json)?;

        let path = self.cache_dir.join("sha1-version").join(hash);
        file::write_str(path, &version.id)?;

        Ok(version)
    }

    pub fn get_versions_from_hashes(&self, hashes: Vec<String>) -> HashMap<String, Version> {
        // println!("{:#?}", hashes);
        let mut sha1_to_version = HashMap::new();

        for hash in &hashes {
            if let Ok(version) = self.get_version_from_hash(hash) {
                sha1_to_version.insert(hash.to_owned(), version);
            }
        }
        // println!("{:#?}", sha1_to_version);
        sha1_to_version
    }

    pub fn update_versions_from_hashes(&self, hashes: Vec<String>) -> HashMap<String, Version> {
        let mut sha1_to_version = HashMap::new();

        for hash in &hashes {
            if let Ok(version) = self.update_version_from_hash(hash) {
                sha1_to_version.insert(hash.to_owned(), version);
            }
        }
        sha1_to_version
    }
}
