use crate::amcm::config::Config;
use crate::amcm::target::Target;
use crate::amcm::data::Data;
use crate::utils::file::{is_path_exist, write_str};
// use crate::CORE;
use std::env::current_exe;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use super::AMCM_DIR;
use super::cache::Cache;

use tokio::sync::Mutex;
lazy_static! {
    pub static ref CORE: Mutex<Core> = Mutex::new(Core::init());
}

use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::Window;
pub struct Core {
    config: Config,
    data: Data,

    config_path: PathBuf,
    data_path: PathBuf,
    storge_dir: PathBuf,

    notify_watcher: Option<RecommendedWatcher>,
}

impl Core {
    pub fn default() -> Core {
        Core {
            config: Config::default(),
            data: Data::default(),
            config_path: AMCM_DIR.join("amcm-config.json"),
            data_path: AMCM_DIR.join("amcm-data.json"),
            storge_dir: AMCM_DIR.join("storage/"),
            notify_watcher: None,
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
        if !is_path_exist(AMCM_DIR.as_path()) {
            fs::create_dir(AMCM_DIR.as_path()).expect(".amcm/ create failed");
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
        write_str(
            &self.config_path,
            &serde_json::to_string(&self.config).unwrap(),
        ).expect("save config failed");
    }
    pub fn save_data(&self) {
        write_str(&self.data_path, &serde_json::to_string(&self.data).unwrap()).expect("save data failed");
    }

    pub fn add_target(&mut self, target: Target) {
        self.config.add_target(target);
        self.save_config();
    }

    pub fn del_target(&mut self, index: usize) {
        self.config.del_target(index);
        self.save_config();
    }

    // pub async fn update_data_from_hash(&mut self, hash: String) -> Result<(), String> {
    //     let res = self.data.update_data_from_hash(hash).await;
    //     self.save_data();
    //     res
    // }

    // pub async fn update_data_from_hashes(&mut self, hashes: Vec<String>) -> Result<(), String> {
    //     let res = self.data.update_data_from_hashes(hashes).await;
    //     self.save_data();
    //     res
    // }

    fn notify_event_handler(res: notify::Result<notify::Event>, window: Window) {
        use notify::{
            event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
            EventKind,
        };
        match res {
            Ok(event) => {
                println!("###### Notify Event handle start #####");

                // Get amcm lock
                let time_event_start = Instant::now();
                let mut amcm = futures::executor::block_on(async { CORE.lock().await });
                println!("get amcm lock cost: {:#?}", time_event_start.elapsed());

                // Update mod file
                let time_start = Instant::now();
                println!("{:#?}", event);
                let path = event.paths.last().unwrap();
                let kind = event.kind;
                if let EventKind::Create(create_kind) = kind {
                    if let CreateKind::File = create_kind {
                        amcm.data()
                            .update_local_mod_file(String::from(path.to_str().unwrap()));
                    } else if let CreateKind::Any = create_kind {
                        amcm.data()
                            .update_local_mod_file(String::from(path.to_str().unwrap()));
                    }
                } else if let EventKind::Modify(modify_kind) = kind {
                    if let ModifyKind::Name(rename_mode) = modify_kind {
                        if let RenameMode::To = rename_mode {
                            amcm.data()
                                .update_local_mod_file(String::from(path.to_str().unwrap()));
                        }
                    }
                } else if let EventKind::Remove(remove_kind) = kind {
                    if let RemoveKind::File = remove_kind {
                        amcm.data()
                            .remove_local_mod_file_from_filepath(String::from(path.to_str().unwrap()));
                    } else if let RemoveKind::Any = remove_kind {
                        amcm.data()
                            .remove_local_mod_file_from_filepath(String::from(path.to_str().unwrap()));
                    }
                } else {
                    println!("Event total cost: {:#?}", time_event_start.elapsed());
                    println!("###### Event handle end #####");
                    return;
                }
                println!("Update data cost: {:#?}", time_start.elapsed());

                // Save data
                let time_start = Instant::now();
                amcm.save_data();
                println!("Save data cost: {:#?}", time_start.elapsed());

                // Emit
                let time_start = Instant::now();
                window
                    .emit("mod_files_updated", amcm.data().local_mod_files())
                    .expect("Event mod_files_updated emit failed");
                println!("Emit cost: {:#?}", time_start.elapsed());

                println!("Event total cost: {:#?}", time_event_start.elapsed());
                println!("###### Notify Event handle end #####");
            }
            Err(e) => {
                println!("watch error: {:?}", e);
            }
        }
    }

    pub async fn watch_mod_files(&mut self, dir: String, window: Window) -> Result<(), String> {
        println!("-> amcm/core.rs/watch_mod_files");
        
        let time_start = std::time::Instant::now();
        // self.data.update_local_mod_files(dir.clone()).await;
        // println!("   update local mod files cost: {:#?}", time_start.elapsed());

        window
            .emit("mod_files_updated", self.data.local_mod_files())
            .expect("Event mod_files_updated emit failed");
        println!("   emit mod_files_updated cost: {:#?}", time_start.elapsed());

        let time_start1 = std::time::Instant::now();
        let window_ = window.clone();
        match recommended_watcher(move |res: notify::Result<notify::Event>| {
            Core::notify_event_handler(res, window_.clone())
        }) {
            Ok(watcher) => self.notify_watcher = Some(watcher),
            Err(error) => {
                return Err(error.to_string());
            }
        }
        println!("   create watcher cost: {:#?}", time_start1.elapsed());

        let time_start1 = std::time::Instant::now();
        let path = Path::new(dir.as_str());
        if let Err(error) = self
            .notify_watcher
            .as_mut()
            .unwrap()
            .watch(path, RecursiveMode::NonRecursive)
        {
            return Err(error.to_string());
        }
        println!("   watch cost: {:#?}", time_start1.elapsed());

        window.once("unwatch_mod_files", move |_| {
            let mut amcm = futures::executor::block_on(async { CORE.lock().await });
            let path = Path::new(dir.as_str());
            amcm.notify_watcher
                .as_mut()
                .unwrap()
                .unwatch(path)
                .expect("Mod files unwatch failed");
        });

        println!("   total cost: {:#?}", time_start.elapsed());
        println!("<- amcm/core.rs/watch_mod_files");
        Ok(())
    }
}
