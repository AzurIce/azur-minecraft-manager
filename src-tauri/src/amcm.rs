use ferinth::structures::version_structs::Version;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
};

use crate::config::{Config, Target};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    hash_to_version: HashMap<String, Version>,
}

impl Data {
    fn default() -> Data {
        Data {
            hash_to_version: HashMap::new(),
        }
    }
}

pub struct Core {
    config: Config,
    data: Data,
}

impl Core {
    pub fn default() -> Core {
        Core {
            config: Config::default(),
            data: Data::default(),
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

    fn load_data(&mut self) {
        self.data = Data::default();

        if let Ok(mut file) = fs::File::open("amcm-data.json") {
            let mut json_str = String::new();
            file.read_to_string(&mut json_str)
                .expect("amcm-data.json read failed");
            self.data = serde_json::from_str(&json_str).unwrap();
        } else {
            let mut file =
                fs::File::create("amcm-data.json").expect("amcm-data.json created failed");
            let json_str = serde_json::to_string(&self.data).unwrap();
            file.write_all(json_str.as_bytes())
                .expect("default data wrote failed")
        }
    }
    
    fn load_config(&mut self) {
        self.config = Config::default();

        if let Ok(mut file) = fs::File::open("amcm-config.json") {
            let mut json_str = String::new();
            file.read_to_string(&mut json_str)
                .expect("amcm-config.json read failed");
            self.config = serde_json::from_str(&json_str).unwrap();
        } else {
            let mut file =
                fs::File::create("amcm-config.json").expect("amcm-config.json created failed");
            let json_str = serde_json::to_string(&self.config).unwrap();
            file.write_all(json_str.as_bytes())
                .expect("default config wrote failed")
        }
    }

    pub fn config(&mut self) -> &mut Config {
        &mut self.config
    }
    pub fn data(&mut self) -> &mut Data {
        &mut self.data
    }

    pub fn save_config(&self) {
        use crate::json;
        json::save_json_str("amcm-config.json", serde_json::to_string(&self.config).unwrap());
    }
    pub fn save_data(&self) {
        use crate::json;
        json::save_json_str("amcm-data.json", serde_json::to_string(&self.data).unwrap());
    }

    pub fn add_target(&mut self, target: Target) {
        self.config.add_target(target);
        self.save_config();
    }

    pub fn del_target(&mut self, index: usize) {
        self.config.del_target(index);
        self.save_config();
    }
}
