use crate::amcm::config::{Config, Target};
use crate::amcm::data::Data;
use crate::utils::file::save_str;
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
    config_dir: PathBuf,
    data_dir: PathBuf,
    // TODO: use hot watch watch the mod dir, and emit events to frontend
    watcher: Hotwatch,
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
            watcher: Hotwatch::new().expect("hotwatch failed to initialize!"),
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

    pub async fn watch_mod_files(&mut self, dir: String, window: Window) -> Result<(), String> {
        
        self.data.update_mod_files(dir.clone()).await;
        window.emit("mod_files_updated", self.data.mod_files());
        let dir1 = dir.clone();
        window.once("unwatch_mod_files", move |_|{
            let mut amcm = futures::executor::block_on(async {
                CORE.lock().await
            });
            amcm.watcher.unwatch(dir1.clone());
        });
        let dir2 = dir.clone();
        let dir3 = dir.clone();
        if let Err(error) = self.watcher.watch(dir2, move |event: Event| {
            let mut amcm = futures::executor::block_on(async {
                CORE.lock().await
            });
            println!("{:#?}", event);futures::executor::block_on(async {
                amcm.data().update_mod_files(dir3.clone()).await;
            });
            window.emit("mod_files_updated", amcm.data().mod_files());
        }) {
            return Err(error.to_string());
        }
        Ok(())
    }
    // pub async fn update_mod_files(&mut self, path: String) {
    //     self.data.update_mod_files(path.clone()).await;
    // }
    // pub fn watch_target(&mut self, target: Target, window: Window) -> Result<(), String> {
    //     self.hotwatch.watch(target.location.clone(), move |event: Event| {
    //         println!("{:#?}", event);
    //         window.emit("mod-files-changed", ());

    //         // if let Event::Create(path) = event {
    //         //     println!("create");
    //         // } else if let Event::Remove(path) = event {
    //         //     println!("remove");
    //         // }
    //     }).expect("failed to watch file!");

    //     Ok(())
    // }

    // pub fn unwatch_target(&mut self, target: Target) -> Result<(), String> {
    //     self.hotwatch.unwatch(target.location);
    //     Ok(())
    // }
}
