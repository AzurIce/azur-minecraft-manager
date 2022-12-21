use ferinth::structures::{project_structs::Project, version_structs::Version};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
};

use crate::{
    config::{Config, Target},
    file,
};

#[derive(Serialize, Deserialize, Debug)]
struct ModFileBelong {
    pub version_id: String,
    pub project_id: String,
}

impl ModFileBelong {
    pub fn new(version_id: String, project_id: String) -> ModFileBelong {
        ModFileBelong {
            version_id,
            project_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    versions: HashMap<String, Version>,
    projects: HashMap<String, Project>,
    hash_to_mfb: HashMap<String, ModFileBelong>,
}

impl Data {
    fn default() -> Data {
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

use std::env::current_exe;
use std::path::PathBuf;
pub struct Core {
    config: Config,
    data: Data,
    config_dir: PathBuf,
    data_dir: PathBuf,
}

impl Core {
    pub fn default() -> Core {
        Core {
            config: Config::default(),
            data: Data::default(),
            config_dir: current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("amcm-config.json"),
            data_dir: current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("amcm-data.json"),
        }
    }

    pub fn init() -> Core {
        let mut core = Core::default();

        core.load();

        core
    }

    pub fn load(&mut self) {
        self.load_config();
        self.load_data();
    }

    fn init_data(&mut self) {
        self.data = Data::default();
        let mut file = fs::File::create(&self.data_dir).expect("amcm-data.json created failed");
        let json_str = serde_json::to_string(&self.data).unwrap();
        file.write_all(json_str.as_bytes())
            .expect("default data wrote failed")
    }
    fn init_config(&mut self) {
        self.config = Config::default();
        let mut file = fs::File::create(&self.config_dir).expect("amcm-config.json created failed");
        let json_str = serde_json::to_string(&self.config).unwrap();
        file.write_all(json_str.as_bytes())
            .expect("default config wrote failed")
    }

    fn load_data(&mut self) {
        if let Ok(mut file) = fs::File::open(&self.data_dir) {
            let mut json_str = String::new();
            file.read_to_string(&mut json_str)
                .expect("amcm-data.json read failed");
            if let Ok(data) = serde_json::from_str(&json_str) {
                self.data = data;
            } else {
                self.init_data();
            }
        } else {
            self.init_data()
        }
    }

    fn load_config(&mut self) {
        if let Ok(mut file) = fs::File::open(&self.config_dir) {
            let mut json_str = String::new();
            file.read_to_string(&mut json_str)
                .expect("amcm-config.json read failed");
            if let Ok(config) = serde_json::from_str(&json_str) {
                self.config = config;
            } else {
                self.init_config();
            }
        } else {
            self.init_config();
        }
    }

    pub fn config(&mut self) -> &mut Config {
        &mut self.config
    }
    pub fn data(&mut self) -> &mut Data {
        &mut self.data
    }

    pub fn save_config(&self) {
        file::save_str(
            &self.config_dir,
            serde_json::to_string(&self.config).unwrap(),
        );
    }
    pub fn save_data(&self) {
        file::save_str(&self.data_dir, serde_json::to_string(&self.data).unwrap());
    }

    pub fn add_target(&mut self, target: Target) {
        self.config.add_target(target);
        self.save_config();
    }

    pub fn del_target(&mut self, index: usize) {
        self.config.del_target(index);
        self.save_config();
    }

    pub async fn update_data_from_hash(&mut self, hash: String) -> Result<(), String> {
        let res = self.data.update_data_from_hash(hash).await;
        self.save_data();
        res
    }

    pub async fn update_data_from_hashes(&mut self, hashes: Vec<String>) -> Result<(), String> {
        let res = self.data.update_data_from_hashes(hashes).await;
        self.save_data();
        res
    }
}
