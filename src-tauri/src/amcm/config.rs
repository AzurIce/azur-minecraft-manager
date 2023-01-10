use serde::{Deserialize, Serialize};
use crate::amcm::structures::mod_source::ModSource;
use crate::amcm::target::Target;


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

    pub fn add_target(&mut self, target: Target) {
        self.targets.push(target);
    }
    pub fn del_target(&mut self, index: usize) {
        self.targets.remove(index);
    }
}
