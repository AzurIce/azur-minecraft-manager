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
            config.save().expect("cannot save config");
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

    ///// mod_source /////
    pub fn add_mod_source(&mut self, project_id: String) -> Result<(), Box<dyn Error>> {
        for mod_source in &self.mod_sources {
            match mod_source {
                ModSource::Modrinth(modrinth_mod_source) => {
                    if modrinth_mod_source.project_id == project_id {
                        return Ok(())
                    }
                },
            }
        }
        self.mod_sources.push(ModSource::Modrinth(ModrinthModSource::new(project_id)));
        self.save()?;
        Ok(())
    }

    pub fn add_mod_source_to_target(&mut self, target_id: usize, project_id: String) -> Result<(), Box<dyn Error>> {
        if let Some(target) = self.targets.get_mut(target_id) {
            target.add_mod_source(ModSource::Modrinth(ModrinthModSource::new(project_id)));
            self.save()?;
        }
        Ok(())
    }



    pub fn add_target(&mut self, target: Target) {
        self.targets.push(target);
        self.save().expect("cannot save config");
    }
    pub fn del_target(&mut self, index: usize) {
        self.targets.remove(index);
        self.save().expect("cannot save config");
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
