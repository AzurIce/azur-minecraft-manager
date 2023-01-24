use std::error::Error;
use std::path::Path;
use std::time::Instant;

// use crate::amcm::data::DATA;
use crate::amcm::structures::mod_file::ModFile;

use tokio::sync::Mutex;
lazy_static! {
    pub static ref CORE: Mutex<Core> = Mutex::new(Core::default());
}

use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::Window;
use tokio::runtime::Runtime;
use tokio::task;
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
                // Update mod file
                let path = event.paths.last().unwrap();
                if path.extension().unwrap() != "jar" && path.extension().unwrap() != "disabled" {
                    return Ok(());
                }
                let kind = event.kind;
                if let EventKind::Create(create_kind) = kind {
                    println!("Create {:#?}", create_kind);
                    if let CreateKind::File = create_kind {
                        let mod_file = ModFile::of(path.as_path());
                        window
                            .emit("mod_file_updated", mod_file)
                            .expect("Event mod_file_updated emit failed");
                    } else if let CreateKind::Any = create_kind {
                        let mod_file = ModFile::of(path.as_path());
                        window
                            .emit("mod_file_updated", mod_file)
                            .expect("Event mod_file_updated emit failed");
                    }
                    return Ok(());
                } else if let EventKind::Modify(modify_kind) = kind {
                    println!("Modify {:#?}", modify_kind);
                    if let ModifyKind::Name(rename_mode) = modify_kind {
                        if let RenameMode::To = rename_mode {
                            let mod_file = ModFile::of(path.as_path());
                            window
                                .emit("mod_file_updated", mod_file)
                                .expect("Event mod_file_updated emit failed");
                        }
                    }
                    return Ok(());
                } else if let EventKind::Remove(remove_kind) = kind {
                    println!("Remove {:#?}", remove_kind);
                    if let RemoveKind::File = remove_kind {
                        // let mf = data.remove_mod_file_from_filepath(path.as_path());
                        window
                            .emit("mod_file_deleted", path.to_str().unwrap())
                            .expect("Event mod_file_deleted emit failed");
                    } else if let RemoveKind::Any = remove_kind {
                        // let mf = data.remove_mod_file_from_filepath(path.as_path());
                        window
                            .emit("mod_file_deleted", path.to_str().unwrap())
                            .expect("Event mod_file_deleted emit failed");
                    }
                    return Ok(());
                } else {
                    return Ok(());
                }

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
        let path = Path::new(dir.as_str()).join(".minecraft").join("mods");
        println!("Watching {:#?}", path);
        if let Err(error) = self
            .notify_watcher
            .as_mut()
            .unwrap()
            .watch(path.as_path(), RecursiveMode::NonRecursive)
        {
            return Err(error.to_string());
        }

        // Listen to unwatch once
        window.once("unwatch_mod_files", move |_| {
            task::block_in_place(|| {
                let mut amcm = CORE.blocking_lock();
                let path = Path::new(dir.as_str());
                amcm.notify_watcher
                    .as_mut()
                    .unwrap()
                    .unwatch(path)
                    .expect("Mod files unwatch failed");
            })
        });

        println!("<- amcm/core.rs/watch_mod_files");
        Ok(())
    }
}
