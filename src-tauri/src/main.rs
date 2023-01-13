#![feature(fs_try_exists)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use amcm::cache::CACHE;

/* MODULESS */
mod utils;
mod amcm;
mod cmd;
// #[cfg(test)]
// mod test;

/* LAZY_STATIC */
#[macro_use]
extern crate lazy_static;

fn main() {
    // {
    //     futures::executor::block_on(CACHE.lock());
    // }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // cmd::load,
            cmd::target::get_target_list,
            cmd::target::get_target,
            cmd::target::add_target,
            cmd::target::del_target,
            cmd::mcmod::get_local_mod_files,

            cmd::mcmod::get_version_from_hash,
            cmd::mcmod::get_versions_from_hashes,
            // cmd::mcmod::update_data_from_hashes,
            // cmd::mcmod::update_data_from_hash,
            // cmd::mcmod::get_version_from_hash,
            // cmd::mcmod::get_project_from_hash,
            // cmd::mcmod::get_project_from_hashes,
            // cmd::mcmod::get_mod_files,
            cmd::mcmod::update_local_mod_files,
            // cmd::mcmod::set_mod_file_enabled,
            cmd::mod_file::enable_mod_file,
            cmd::mod_file::disable_mod_file,

            cmd::mod_source::add_mod_source,
            cmd::mod_source::add_mod_source_from_local_mod_file,
            cmd::mod_source::add_mod_source_to_target,

            // cmd::mcmod::enter_manage_target,
            // cmd::mcmod::leave_manage_target,
            cmd::mcmod::watch_mod_files,
            cmd::mcmod::target_watch_mod_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
