use sha::sha1::Sha1;
use sha::utils::{Digest, DigestExt};
use std::{fs::{self, File}, io::{Read, Write}};
use std::path::Path;


// Sha1 //
pub fn get_file_sha1<P: AsRef<Path>>(path: P) -> String {
    let bytes = fs::read(path).unwrap();
    Sha1::default().digest(&bytes[..]).to_hex()
}

// exist //
pub fn is_path_exist<P: AsRef<Path>>(path: P) -> bool {
    return fs::try_exists(path).unwrap_or(false);
}

// IO //
use std::io::Result;
#[tauri::command]
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn write_str<P: AsRef<Path>>(path: P, data: &str) -> Result<()> {
    fs::create_dir_all(path.as_ref().parent().unwrap())?;
    let mut file = File::create(path)?;
    file.write((*data).as_bytes())?;
    Ok(())
}

pub fn copy_file<P: AsRef<Path>>(src: P, dst: P) -> Result<()> {
    fs::create_dir_all(dst.as_ref().parent().unwrap())?;
    fs::copy(src, dst)?;
    Ok(())
}

pub fn delete_file<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::remove_file(path)
}
