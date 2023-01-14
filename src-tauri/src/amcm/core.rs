use std::error::Error;
use std::path::Path;
use std::time::Instant;

use crate::amcm::data::DATA;

use tokio::sync::Mutex;
lazy_static! {
    pub static ref CORE: Mutex<Core> = Mutex::new(Core::default());
}

use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::Window;
use tokio::runtime::Runtime;
pub struct Core {
    notify_watcher: Option<RecommendedWatcher>,
}

impl Core {
    pub fn default() -> Core {
        Core {
            notify_watcher: None,
        }
    }

    fn notify_event_handler(
        res: notify::Result<notify::Event>,
        window: Window,
    ) -> Result<(), Box<dyn Error>> {
        use notify::{
            event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
            EventKind,
        };
        match res {
            Ok(event) => {
                // Get data lock
                let mut data = DATA.blocking_lock();
                
                // Update mod file
                let path = event.paths.last().unwrap();
                if path.extension().unwrap() != "jar" && path.extension().unwrap() != "disabled"
                {
                    return Ok(());
                }
                let kind = event.kind;
                if let EventKind::Create(create_kind) = kind {
                    if let CreateKind::File = create_kind {
                        data.update_mod_file(path.as_path());
                    } else if let CreateKind::Any = create_kind {
                        data.update_mod_file(path.as_path());
                    }
                } else if let EventKind::Modify(modify_kind) = kind {
                    if let ModifyKind::Name(rename_mode) = modify_kind {
                        if let RenameMode::To = rename_mode {
                            data.update_mod_file(path.as_path());
                        }
                    }
                } else if let EventKind::Remove(remove_kind) = kind {
                    if let RemoveKind::File = remove_kind {
                        data.remove_mod_file_from_filepath(path.as_path());
                    } else if let RemoveKind::Any = remove_kind {
                        data.remove_mod_file_from_filepath(path.as_path());
                    }
                } else {
                    return Ok(());
                }

                // Emit
                let time_start = Instant::now();
                window
                    .emit("mod_files_updated", data.mod_files())
                    .expect("Event mod_files_updated emit failed");
                Ok(())
            }
            Err(e) => {
                println!("watch error: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    pub async fn watch_mod_files(&mut self, dir: String, window: Window) -> Result<(), String> {
        println!("-> amcm/core.rs/watch_mod_files");

        // Init watcher
        let window_ = window.clone();
        match recommended_watcher(move |res: notify::Result<notify::Event>| {
            Core::notify_event_handler(res, window_.clone()).unwrap()
        }) {
            Ok(watcher) => self.notify_watcher = Some(watcher),
            Err(error) => {
                return Err(error.to_string());
            }
        }

        // Watch
        let path = Path::new(dir.as_str());
        if let Err(error) = self
            .notify_watcher
            .as_mut()
            .unwrap()
            .watch(path, RecursiveMode::NonRecursive)
        {
            return Err(error.to_string());
        }

        // Listen to unwatch once
        window.once("unwatch_mod_files", move |_| {
            let mut amcm = CORE.blocking_lock();
            let path = Path::new(dir.as_str());
            amcm.notify_watcher
                .as_mut()
                .unwrap()
                .unwatch(path)
                .expect("Mod files unwatch failed");
        });

        println!("<- amcm/core.rs/watch_mod_files");
        Ok(())
    }
}
