
use std::fs;
use std::io::{Read, Write};
use std::env::current_exe;
use std::path::PathBuf;
use crate::utils::file::save_str;
use crate::amcm::data::Data;
use crate::amcm::config::{Config, Target};
pub struct Core {
    config: Config,
    data: Data,
    config_dir: PathBuf,
    data_dir: PathBuf,
    // TODO: use hot watch watch the mod dir, and emit events to frontend
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
        save_str(
            &self.config_dir,
            serde_json::to_string(&self.config).unwrap(),
        );
    }
    pub fn save_data(&self) {
        save_str(&self.data_dir, serde_json::to_string(&self.data).unwrap());
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
