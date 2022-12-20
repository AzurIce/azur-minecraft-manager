#![feature(fs_try_exists)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/* MODULESS */

mod file;
mod json;
mod cmd;
mod config;
mod modrinth;
mod modloader;
mod mcmod;
mod amcm;

// use api::core::{Core, Target, TargetKind};

use amcm::Core;
use std::sync::Mutex;
/* LAZY_STATIC */
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CORE: Mutex<Core> = Mutex::new(Core::init());
}
/* ONCE_CELL */
// use once_cell::sync::Lazy;

// static CORE: Mutex<Core>

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn get_target_list() -> String {
//     CORE.config.get_target_list()
// }

// #[tauri::command]
// fn add_target(target_json_str: String) {
//     let target: Target = Target::from_str(target_json_str);
//     // CORE.add_target(target);
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            file::is_path_exist,
            json::get_json_str,
            json::save_json_str,
            cmd::target::get_target_list,
            cmd::target::get_target,
            cmd::target::add_target,
            cmd::target::del_target,
            cmd::mcmod::get_mod_file_list,
            cmd::mcmod::get_mod_filename_list,
            // cmd::mcmod::get_belonged_mod_file,
            // api::mcmod::get_mod_filename_list,
            // config::get_mod_file_list,
            // config::get_mod_file,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
