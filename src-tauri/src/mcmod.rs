use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::file;

#[derive(Serialize, Deserialize, Debug)]
pub enum ModLoader {
    Forge,
    Fabric,
    Quilt,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ModEnv {
    Client,
    Server,
    Both,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ModSource {
    Modrinth(String),
    Unknown
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mod {
    pub source: ModSource,
    pub name: String,
    pub desc: String,
    pub env: ModEnv,
}

impl Mod {
    pub fn default() -> Mod {
        Mod {
            source: ModSource::Unknown,
            name: String::from(""),
            desc: String::from(""),
            env: ModEnv::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModFile {
    pub filename: String, // From file
    pub sha1: String,

    pub belong_mod: Mod, // From sha1 + modrinth API
    pub game_versions: Vec<String>,
}

impl ModFile {
    pub fn default() -> ModFile {
        ModFile {
            filename: String::from(""),
            sha1: String::from(""),
            belong_mod: Mod::default(),
            game_versions: Vec::new(),
        }
    }

    pub fn of(path: PathBuf) -> ModFile {
        let mut mod_file = ModFile::default();

        mod_file = ModFile {
            filename: String::from(path.file_name().unwrap().to_str().unwrap()),
            sha1: file::get_file_sha1(path.to_str().unwrap()),
            ..mod_file
        };

        mod_file
    }
}

use ferinth::{Ferinth, Error};
use crate::modrinth;
pub async fn get_belonged_mod_file(path: PathBuf) -> Result<ModFile, Error> {
    let mut mod_file = ModFile::of(path);
    println!("{:#?} {:#?}", mod_file.filename, mod_file.sha1);

    let modrinth = Ferinth::default();
    // Get version from hash
    let version = modrinth.get_version_from_hash(&mod_file.sha1).await?;
    let game_versions = version.game_versions;

    // Get project from id
    if let Ok(belong_mod) = modrinth::get_mod(version.project_id).await {
        mod_file = ModFile {
            belong_mod,
            game_versions,
            ..mod_file
        };
    }

    // Informations from file's zipped content
    // let reader = BufReader::new(file);
    // let mut archive = zip::ZipArchive::new(reader).unwrap();
    // if let Ok(mut fabric_mod_json_file) = archive.by_name("fabric.mod.json") {
    //     let mut buf = String::new();
    //     fabric_mod_json_file.read_to_string(&mut buf).expect("Read fabric.mod.json failed");
    //     if let Ok(fabric_mod_json) = serde_json::from_str(&buf) as Result<modloader::ModLoaderFile> {
    //         mod_file = ModFile {
    //             mod_id: fabric_mod_json.id,
    //             mod_version: fabric_mod_json.version,
    //             ..mod_file
    //         };
    //     }
    // }

    Ok(mod_file)
}