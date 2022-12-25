#![feature(fs_try_exists)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/* MODULESS */
mod utils;
// mod json;
mod cmd;
mod amcm;
#[cfg(test)]
mod test;

/* LAZY_STATIC */
use tokio::sync::Mutex;
use amcm::core::Core;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CORE: Mutex<Core> = Mutex::new(Core::init());
}

fn main() {
    tauri::Builder::default().invoke_handler(tauri::generate_handler![
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
            cmd::mcmod::get_version_from_hashes,
            cmd::mcmod::get_project_from_hashes,
            cmd::mcmod::get_mod_files,
            cmd::mcmod::update_mod_files,
            // cmd::mcmod::enter_manage_target,
            // cmd::mcmod::leave_manage_target,
            cmd::mcmod::watch_mod_files,
            cmd::mcmod::target_watch_mod_files,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
