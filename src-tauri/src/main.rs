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

/* LAZY_STATIC */
use std::sync::Mutex;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CORE: Mutex<amcm::Core> = Mutex::new(amcm::Core::init());
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // greet,
            // file::is_path_exist,
            // json::get_json_str,
            // json::save_json_str,
            cmd::load,
            cmd::target::get_target_list,
            cmd::target::get_target,
            cmd::target::add_target,
            cmd::target::del_target,
            cmd::mcmod::get_mod_file_list,
            cmd::mcmod::update_data_from_hashes,
            cmd::mcmod::update_data_from_hash,
            cmd::mcmod::get_version_from_hash,
            cmd::mcmod::get_project_from_hash,
            // cmd::mcmod::get_mod_filename_list,
            // cmd::mcmod::get_belonged_mod_file,
            // api::mcmod::get_mod_filename_list,
            // config::get_mod_file_list,
            // config::get_mod_file,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
