use std::fs;
use sha::sha1::Sha1;
use sha::utils::{Digest, DigestExt};

#[tauri::command]
pub fn is_path_exist(path: &str) -> bool {
    return fs::try_exists(path).unwrap_or(false);
}

pub fn get_file_sha1(path: &str) -> String {
    let bytes = fs::read(path).unwrap();
    Sha1::default().digest(&bytes[..]).to_hex()
}