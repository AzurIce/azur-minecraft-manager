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

use amcm::{database::init_db, AMCM_DIR};
use futures::executor::block_on;

// fn init_dir() {
//     use std::fs;
//     fs::
// }

fn main() {
    // init_dir();
    // Initialize the database
    // if let Err(err) = block_on(init_db()) {
    //     panic!("{}", err)
    // }
    // Initialize tauri
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            cmd::get_remote_mods,
            cmd::add_remote_mod,
            cmd::copy_file,
            cmd::rename_file,
            cmd::delete_file,
            // cmd::load,
            // cmd::target::get_targets,
            // cmd::target::get_target,
            // cmd::target::add_target,
            // cmd::target::del_target,
            // cmd::target::copy_version_to_target,
            cmd::version::get_version_from_hash,
            cmd::version::get_versions_from_hashes,
            cmd::version::get_version,
            cmd::version::get_versions,
            cmd::version::is_version_downloaded,
            cmd::version::download_version,
            cmd::project::get_project,
            cmd::project::update_project,
            cmd::mod_file::get_mod_files,
            cmd::mod_file::update_mod_files,
            cmd::mod_file::remove_mod_file,
            cmd::mod_file::delete_mod_file,
            cmd::mod_file::enable_mod_file,
            cmd::mod_file::disable_mod_file,
            cmd::mod_file::watch_mod_files,
            // cmd::mod_file::target_watch_mod_files,
            // cmd::mod_source::add_mod_source,
            // cmd::mod_source::add_mod_source_from_local_mod_file,
            // cmd::mod_source::add_mod_source_to_target,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
