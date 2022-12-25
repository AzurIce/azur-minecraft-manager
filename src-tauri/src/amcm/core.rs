use crate::amcm::config::{Config, Target};
use crate::amcm::data::Data;
use crate::utils::file::{save_str, is_path_exist};
use crate::CORE;
use std::env::current_exe;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use hotwatch::{Event, Hotwatch};
// use notify::{Watcher, RecommendedWatcher, RecursiveMode};
use tauri::Window;
pub struct Core {
    config: Config,
    data: Data,

    amcm_dir: PathBuf,
    config_path: PathBuf,
    data_path: PathBuf,
    storge_dir: PathBuf,

    watcher: Hotwatch,
}

impl Core {
    pub fn default() -> Core {
        let amcm_dir = current_exe().unwrap().parent().unwrap().join(".amcm/");
        Core {
            config: Config::default(),
            data: Data::default(),
            config_path: amcm_dir.join("amcm-config.json"),
            data_path: amcm_dir.join("amcm-data.json"),
            storge_dir: amcm_dir.join("storage/"),
            amcm_dir,
            watcher: Hotwatch::new().expect("hotwatch failed to initialize!"),
        }
    }

    
    pub fn init() -> Core {
        let mut core = Core::default();
        
        core.init_dir();
        
        core.load();
        
        core
    }
    
    pub fn load(&mut self) {
        self.load_config();
        self.load_data();
    }

    fn init_dir(&self) {
        if !is_path_exist(self.amcm_dir.clone()) {
            fs::create_dir(self.amcm_dir.clone()).expect(".amcm/ create failed");
        }
        if !is_path_exist(self.storge_dir.clone()) {
            fs::create_dir(self.storge_dir.clone()).expect(".amcm/storage/ create failed");
        }
    }

    fn init_default_data(&mut self) {
        self.data = Data::default();
        let mut file = fs::File::create(&self.data_path).expect("amcm-data.json created failed");
        let json_str = serde_json::to_string(&self.data).unwrap();
        file.write_all(json_str.as_bytes())
            .expect("default data wrote failed")
    }
    fn init_default_config(&mut self) {
        self.config = Config::default();
        let mut file =
            fs::File::create(&self.config_path).expect("amcm-config.json created failed");
        let json_str = serde_json::to_string(&self.config).unwrap();
        file.write_all(json_str.as_bytes())
            .expect("default config wrote failed")
    }

    fn load_data(&mut self) {
        if let Ok(mut file) = fs::File::open(&self.data_path) {
            let mut json_str = String::new();
            file.read_to_string(&mut json_str)
                .expect("amcm-data.json read failed");
            if let Ok(data) = serde_json::from_str(&json_str) {
                self.data = data;
                return;
            }
        }
        self.init_default_data();
    }

    fn load_config(&mut self) {
        if let Ok(mut file) = fs::File::open(&self.config_path) {
            let mut json_str = String::new();
            file.read_to_string(&mut json_str)
                .expect("amcm-config.json read failed");
            if let Ok(config) = serde_json::from_str(&json_str) {
                self.config = config;
                return;
            }
        }
        self.init_default_config();
    }

    pub fn config(&mut self) -> &mut Config {
        &mut self.config
    }
    pub fn data(&mut self) -> &mut Data {
        &mut self.data
    }

    pub fn save_config(&self) {
        save_str(
            &self.config_path,
            serde_json::to_string(&self.config).unwrap(),
        );
    }
    pub fn save_data(&self) {
        save_str(&self.data_path, serde_json::to_string(&self.data).unwrap());
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

    pub async fn watch_mod_files(&mut self, dir: String, window: Window) -> Result<(), String> {
        // TODO: Optimize it
        self.data.update_mod_files(dir.clone()).await;
        window.emit("mod_files_updated", self.data.mod_files()).expect("Event mod_files_updated emit failed");

        let dir_ = dir.clone();
        let window_ = window.clone();
        if let Err(error) = self.watcher.watch(dir.clone(), move |event: Event| {
            println!("###### Event handle start #####");
            let mut amcm = futures::executor::block_on(async { CORE.lock().await });

            let time_start = std::time::Instant::now();
            println!("{:#?}", event);
            if let Event::Create(filepath)= event {
                amcm.data().update_mod_file(String::from(filepath.to_str().unwrap()));
            } else if let Event::Rename(srcpath, dstpath) = event {
                amcm.data().update_mod_file(String::from(dstpath.to_str().unwrap()));
            } else if let Event::Remove(filepath) = event {
                amcm.data().remove_mod_file_from_filepath(String::from(filepath.to_str().unwrap()));
            } else {
                println!("###### Event handle end #####");
                return;
            }
            println!("Event handle cost: {:#?}", time_start.elapsed());
            let time_start = std::time::Instant::now();
            window_.emit("mod_files_updated", amcm.data().mod_files()).expect("Event mod_files_updated emit failed");
            println!("Emit cost: {:#?}", time_start.elapsed());
            println!("###### Event handle end #####");
        }) {
            return Err(error.to_string());
        }
        let dir_ = dir.clone();
        window.once("unwatch_mod_files", |_| {
            let mut amcm = futures::executor::block_on(async { CORE.lock().await });
            amcm.watcher.unwatch(dir_).expect("Mod files unwatch failed");
        });

        Ok(())
    }
}
