use std::fmt::format;
use std::fs;

use crate::amcm::structures::mods_json::ModsJson;
use crate::utils::file;

pub mod mod_file;
// pub mod mod_source;
pub mod project;
// pub mod target;
pub mod version;

// #[cfg(test)]
// pub mod mod_source_test;

use std::path::Path;

#[tauri::command]
pub fn get_remote_mods(target_dir: String) -> Result<Vec<String>, String> {
    let path = Path::new(&target_dir).join(".amcm").join("mods.json");

    match fs::read_to_string(path) {
        Ok(json_string) => {
            // let mods_json: Result<ModsJson, serde_json::Error> =
            //     serde_json::from_str(json_string.as_str());
            match serde_json::from_str(json_string.as_str()) {
                Ok(res) => Ok(res),
                Err(err) => Err(format!("{}", err)),
            }
        }
        Err(err) => Err(format!("{}", err)),
    }
}
// use crate::amcm::core::CORE;

#[tauri::command]
pub fn add_remote_mod(target_dir: String, project_id: String) -> Result<(), String> {
    let mut remote_mods = get_remote_mods(target_dir.clone()).unwrap_or(Vec::new());
    println!("{:#?}", remote_mods);

    if remote_mods.contains(&project_id) {
        return Err("Already exist!".to_owned());
    }

    remote_mods.push(project_id);

    let path = Path::new(&target_dir).join(".amcm").join("mods.json");
    let json = serde_json::to_string_pretty(&remote_mods).unwrap();
    file::write_str(path, &json).unwrap();
    Ok(())
}

#[tauri::command]
pub fn copy_file(src: String, dst: String) -> Result<(), String> {
    match file::copy_file(src, dst) {
        Ok(_) => {
            println!("copied file");
            Ok(())
        },
        Err(err) => Err(format!("{}", err)),
    }
}

#[tauri::command]
pub fn delete_file(path: String) -> Result<(), String> {
    match file::delete_file(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}

#[tauri::command]
pub fn rename_file(src: String, dst: String) -> Result<(), String> {
    match fs::rename(src, dst) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}