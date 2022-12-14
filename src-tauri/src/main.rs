#![feature(fs_try_exists)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/* MODULESS */

mod file;
mod json;
mod amcm;

use amcm::core::Core;

/* LAZY_STATIC */
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CORE: Core = Core::init();
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust! {:#?}", name, *CORE)
}

#[tauri::command]
fn get_target_list() -> String {
    CORE.config.get_target_list()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            file::is_path_exist,
            json::get_json_str,
            json::save_json_str,
            get_target_list
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
