// use crate::amcm::structures::BelongState;
use crate::amcm::structures::mod_file::ModFile;
use ferinth::structures::{project::Project, version::Version};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use tokio::sync::Mutex;
lazy_static! {
    pub static ref DATA: Mutex<Data> = Mutex::new(Data::default());
}

// 运行时数据
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    // versions: HashMap<String, Version>,
    // projects: HashMap<String, Project>,
    // hash_to_mfb: HashMap<String, ModFileBelong>,
    // modrinth_sha1s: HashSet<String>,
    local_mod_files: Vec<ModFile>,
}

impl Data {
    pub fn default() -> Data {
        Data {
            // versions: HashMap::new(),
            // projects: HashMap::new(),
            // modrinth_sha1s: HashSet::new(),
            // hash_to_mfb: HashMap::new(),
            local_mod_files: Vec::new(),
        }
    }

    pub async fn update_local_mod_files<P: AsRef<Path>>(&mut self, path: P) -> Vec<ModFile> {
        println!("\n-> amcm/data.rs/update_mod_files: {:#?}", path.as_ref());
        let mut mod_file_list: Vec<ModFile> = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let file_path = entry.unwrap().path();

            if file_path.is_file()
                && !file_path.starts_with("_amcm_")
                && file_path.extension().unwrap() == "jar"
                || file_path.extension().unwrap() == "disabled"
            {
                mod_file_list.push(ModFile::of(&file_path));
            }
        }
        self.local_mod_files = mod_file_list.clone();
        println!("<- amcm/data.rs/update_mod_files\n");
        mod_file_list
    }

    pub fn update_local_mod_file<P: AsRef<Path>>(&mut self, path: P) {
        println!("\n-> data.rs/update_mod_file");
        use std::time::Instant;

        let time_start = Instant::now();
        let new_local_mod_file = ModFile::of(path);
        println!("   mod_file_of cost: {:#?}", time_start.elapsed());

        let time_start = Instant::now();
        for mod_file in &mut self.local_mod_files {
            if mod_file.sha1 == new_local_mod_file.sha1 {
                println!("   Find mod_file cost: {:#?}", time_start.elapsed());
                let time_start = Instant::now();
                *mod_file = new_local_mod_file;
                println!("   Assign mod_file cost: {:#?}", time_start.elapsed());
                println!("<- data.rs/update_mod_file");
                return;
            }
        }
        self.local_mod_files.push(new_local_mod_file);

        println!("<- data.rs/update_mod_file\n");
    }
    pub fn remove_local_mod_file_from_filepath<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref();
        for (index, local_mod_file) in self.local_mod_files.iter().enumerate() {
            if Path::new(&local_mod_file.path) == path {
                self.local_mod_files.remove(index);
                return;
            }
        }
    }

    pub fn local_mod_files(&self) -> Vec<ModFile> {
        println!("-> data.rs/local_mod_files");
        use std::time::Instant;
        let time_start = Instant::now();
        let res = self.local_mod_files.clone();
        println!("   local_mod_files.clone() cost: {:#?}", time_start.elapsed());
        println!("<- data.rs/local_mod_files\n");
        res
    }

    pub fn get_mod_file_from_hash(&self, hash: String) -> Option<ModFile> {
        for mod_file in &self.local_mod_files {
            if mod_file.sha1 == hash {
                return Some((*mod_file).clone());
            }
        }
        None
    }

}
