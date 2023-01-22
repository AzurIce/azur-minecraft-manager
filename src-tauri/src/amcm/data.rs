// use crate::amcm::structures::BelongState;
use crate::amcm::structures::mod_file::ModFile;
// use crate::amcm::RT;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

use tokio::task;

use tokio::sync::Mutex;
lazy_static! {
    pub static ref DATA: Mutex<Data> = Mutex::new(Data::default());
}

// 运行时数据
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    mod_files: Vec<ModFile>,
}

impl Data {
    pub fn default() -> Data {
        Data {
            mod_files: Vec::new(),
        }
    }

    pub fn mod_files(&self) -> Vec<ModFile> {
        self.mod_files.clone()
    }

    pub fn update_mod_files<P: AsRef<Path>>(&mut self, path: P) -> Vec<ModFile> {
        println!("\n-> amcm/data.rs/update_mod_files: {:#?}", path.as_ref());
        let mut mod_files: Vec<ModFile> = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let file_path = entry.unwrap().path();

            if file_path.is_file()
                && (file_path.extension().unwrap() == "jar"
                    || file_path.extension().unwrap() == "disabled")
            {
                mod_files.push(ModFile::of(&file_path));
            }
        }
        self.mod_files = mod_files.clone();
        println!("<- amcm/data.rs/update_mod_files\n");
        mod_files
    }

    pub fn update_mod_file<P: AsRef<Path>>(&mut self, path: P) -> ModFile {
        let new_mod_file = ModFile::of(path);

        for mod_file in &mut self.mod_files {
            if mod_file.sha1 == new_mod_file.sha1 {
                *mod_file = new_mod_file.clone();
                return new_mod_file;
            }
        }
        self.mod_files.push(new_mod_file.clone());
        new_mod_file
    }

    pub fn remove_mod_file_from_filepath<P: AsRef<Path>>(&mut self, path: P) -> ModFile {
        let path = path.as_ref();
        for (index, mod_file) in self.mod_files.iter().enumerate() {
            if Path::new(&mod_file.path) == path {
                let mf = self.mod_files.get(index).unwrap().clone();
                self.mod_files.remove(index);
                return mf;
            }
        }
        return ModFile::default();
    }

    pub fn get_mod_file_from_hash(&self, hash: String) -> Option<ModFile> {
        for mod_file in &self.mod_files {
            if mod_file.sha1 == hash {
                return Some((*mod_file).clone());
            }
        }
        None
    }

    pub fn remove_mod_file_from_hash(&mut self, hash: String) -> Result<(), Box<dyn Error>> {
        for (index, &ref mod_file) in self.mod_files.iter().enumerate() {
            if mod_file.sha1 == hash {
                self.mod_files.remove(index);
                break;
            }
        }
        Ok(())
    }
}
