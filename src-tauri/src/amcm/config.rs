use serde::{Deserialize, Serialize};
use crate::amcm::structures::mod_source::{ModSource, ModrinthModSource};
use crate::amcm::target::Target;
use crate::utils::file;
use super::AMCM_DIR;
use std::error::Error;
use std::fs;
use std::path::PathBuf;


use tokio::sync::Mutex;
lazy_static! {
    pub static ref CONFIG_PATH: PathBuf = AMCM_DIR.join("config.json");
    pub static ref CONFIG: Mutex<Config> = {
        let mut config = Config::default();

        if let Err(err) = config.load() {
            println!("[ERROR] Init config: {}, initializing default config...", err);
            config.save();
        }

        Mutex::new(config)
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    targets: Vec<Target>,
    mod_sources: Vec<ModSource>
}

impl Config {
    pub fn default() -> Config {
        Config {
            targets: Vec::new(),
            mod_sources: Vec::new(),
        }
    }

    pub fn targets(&self) -> Vec<Target> {
        self.targets.clone()
    }
    pub fn get_target(&self, index: usize) -> Result<Target, String> {
        if let Some(target) = self.targets.get(index) {
            Ok(target.clone())
        } else {
            Err(String::from("Index out of bound"))
        }
    }

    pub fn add_mod_source_to_target(&mut self, target_id: usize, project_id: String) -> Result<(), String> {
        if let Some(target) = self.targets.get_mut(target_id) {
            target.add_mod_source(ModSource::Modrinth(ModrinthModSource::new(project_id)));
            self.save();
            Ok(())
        } else {
            Err(String::from("Index out of bound"))
        }
    }

    pub fn add_target(&mut self, target: Target) {
        self.targets.push(target);
        self.save();
    }
    pub fn del_target(&mut self, index: usize) {
        self.targets.remove(index);
        self.save();
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(self)?;
        file::write_str(CONFIG_PATH.as_path(), &json)?;
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let json = fs::read_to_string(CONFIG_PATH.as_path())?;
        *self = serde_json::from_str(&json)?;
        Ok(())
    }
}
