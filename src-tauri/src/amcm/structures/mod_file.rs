use std::{error::Error, fs, path::Path};

use ferinth::structures::version::Version;
use serde::{Deserialize, Serialize};
use tokio::task;

use crate::{amcm::AMCM_DIR, utils::file};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModFile {
    pub path: String,
    pub filename: String,
    pub sha1: String,
    pub enabled: bool,
}

impl ModFile {
    pub fn default() -> ModFile {
        ModFile {
            path: String::from(""),
            filename: String::from(""),
            sha1: String::from(""),
            enabled: true,
        }
    }

    pub fn of<P: AsRef<Path>>(path: P) -> ModFile {
        // task::block_in_place(|| {
            use std::time::Instant;
            let time_start = Instant::now();
            let path = path.as_ref();
            let filename = String::from(path.file_name().unwrap().to_str().unwrap());
            let sha1 = file::get_file_sha1(path.to_str().unwrap());
            let enabled = path.extension().unwrap() == "jar";
            println!(
                "       get file attributes cost: {:#?}",
                time_start.elapsed()
            );


            println!("   mod_file_of cost: {:#?}", time_start.elapsed());
            ModFile {
                path: String::from(path.to_str().unwrap()),
                filename,
                sha1,
                enabled,
            }
    }
}

pub fn update_mod_files<P: AsRef<Path>>(path: P) -> Vec<ModFile> {
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
    println!("<- amcm/data.rs/update_mod_files\n");
    mod_files
}