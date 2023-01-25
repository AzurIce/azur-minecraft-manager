#![feature(fs_try_exists)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/* LAZY_STATIC */
#[macro_use]
extern crate lazy_static;

/* MODULESS */
mod amcm;
mod cmd;
mod utils;

fn main() {
    // Initialize tauri
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            cmd::get_remote_mods,
            cmd::add_remote_mod,
            cmd::copy_file,
            cmd::rename_file,
            cmd::delete_file,
            cmd::version::get_version_from_hash,
            cmd::version::get_version,
            cmd::version::get_versions,
            cmd::version::is_version_downloaded,
            cmd::version::download_version,
            cmd::project::get_project,
            cmd::project::get_projects,
            cmd::mod_file::update_mod_files,
            cmd::mod_file::watch_mod_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
