pub mod core;
pub mod structures;

use tauri::api::path;

use std::path::PathBuf;
use std::env;
lazy_static! {
    pub static ref AMCM_DIR: PathBuf = path::data_dir().unwrap()
        .join(".amcm/");
}