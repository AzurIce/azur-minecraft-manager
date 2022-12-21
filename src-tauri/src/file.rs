use sha::sha1::Sha1;
use sha::utils::{Digest, DigestExt};
use std::{fs::{self, File}, io::{Read, Write}};
use std::path::Path;

#[tauri::command]
pub fn is_path_exist(path: &str) -> bool {
    return fs::try_exists(path).unwrap_or(false);
}

pub fn get_file_sha1(path: &str) -> String {
    let bytes = fs::read(path).unwrap();
    Sha1::default().digest(&bytes[..]).to_hex()
}

#[tauri::command]
pub fn read_str<P: AsRef<Path>>(path: P) -> String{
    if let Ok(mut file) = File::open(path) {
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Read json failed");
        return buf;
    } else {
        return String::from("{}");
    }
}

#[tauri::command]
pub fn save_str<P: AsRef<Path>>(path: P, data: String) {
    let mut file = File::create(path).expect("Create json failed");

    file.write(data.as_bytes()).expect("Write json failed");
}
