#![feature(fs_try_exists)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/* MODULESS */

mod file;
mod json;
mod api;
mod config;
mod modrinth;
mod modloader;

// use api::core::{Core, Target, TargetKind};

/* LAZY_STATIC */
// #[macro_use]
// extern crate lazy_static;

// lazy_static! {
//     static ref CORE: Core = Core::init();
// }

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
            api::target::get_target_list_json,
            api::target::get_target_json,
            api::target::add_target,
            api::mcmod::get_mod_filename_list,
            config::get_mod_file_list,
            config::get_mod_file,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
