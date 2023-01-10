use serde::{Serialize, Deserialize};
use sha::sha1::Sha1;
use sha::utils::{Digest, DigestExt};
use std::{fs::{self, File}, io::{Read, Write}};
use std::path::Path;


// Sha1 //
pub fn get_file_sha1<P: AsRef<Path>>(path: P) -> String {
    let bytes = fs::read(path).unwrap();
    Sha1::default().digest(&bytes[..]).to_hex()
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Sha1File {
//     pub filename: String,
//     pub path: String,
//     pub sha1: String,
// }

// impl Sha1File {
//     pub async fn of<P: AsRef<Path>>(path: P) -> Sha1File {
//         let path = path.as_ref();
//         let filename = String::from(path.file_name().unwrap().to_str().unwrap());
//         let sha1 = get_file_sha1(path.to_str().unwrap());
//         let path = path.to_str().unwrap().to_owned();

//         Sha1File {
//             filename,
//             path,
//             sha1,
//         }
//     }
// }

// exist //
#[tauri::command]
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
