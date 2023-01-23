pub mod core;
pub mod data;
pub mod structures;
// pub mod config;
// pub mod target;
pub mod cache;
pub mod database;
pub mod migrator;

#[cfg(test)]
mod cache_test;

use tauri::api::path;

use std::path::PathBuf;
use std::env;
lazy_static! {
    pub static ref AMCM_DIR: PathBuf = path::data_dir().unwrap()
        .join(".amcm/");
}