use std::fs::{self, File};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::file;
use crate::modloader;
use crate::modrinth;
use std::io::BufReader;
use std::io::Read;
use serde_json::error::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum ModSource {
    Modrinth,
}

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
pub struct Mod {
    pub source: ModSource,
    pub id: String,
    pub mod_name: String, // title field of Modrinth project
    pub mod_description: String,
    pub mod_env: ModEnv,
}

impl Mod {
    pub fn default() -> Mod {
        Mod {
            source: ModSource::Modrinth,
            id: String::from(""),
            mod_name: String::from(""),
            mod_description: String::from(""),
            mod_env: ModEnv::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModFile {
    filename: String, // From file
    sha1: String,
    belong_mod: Mod, // From sha1 + modrinth API
    mod_loader: ModLoader, // From file's zipped content.
    game_version: String,
    mod_id: String,
    mod_version: String,
}

impl ModFile {
    pub fn default() -> ModFile {
        ModFile {
            filename: String::from(""),
            sha1: String::from(""),
            belong_mod: Mod::default(),
            game_version: String::from(""),
            mod_id: String::from(""),
            mod_version: String::from(""),
            mod_loader: ModLoader::Unknown,
        }
    }
}

#[tauri::command]
pub async fn get_mod_file(path: String) -> ModFile {
    let path = Path::new(&path);
    let file = File::open(path).unwrap();

    let mut mod_file = ModFile::default();

    // Informations from file
    mod_file = ModFile {
        filename: String::from(path.file_name().unwrap().to_str().unwrap()),
        sha1: file::get_file_sha1(path.to_str().unwrap()),
        ..mod_file
    };
    println!("{:#?} {:#?}", mod_file.filename, mod_file.sha1);

    // Informations from Modrinth API
    if let Ok(belong_mod) = modrinth::get_mod_from_hash(mod_file.sha1.clone()).await {
        mod_file = ModFile {
            belong_mod,
            ..mod_file
        };
    }

    // Informations from file's zipped content
    let reader = BufReader::new(file);
    let mut archive = zip::ZipArchive::new(reader).unwrap();
    if let Ok(mut fabric_mod_json_file) = archive.by_name("fabric.mod.json") {
        let mut buf = String::new();
        fabric_mod_json_file.read_to_string(&mut buf).expect("Read fabric.mod.json failed");
        if let Ok(fabric_mod_json) = serde_json::from_str(&buf) as Result<modloader::ModLoaderFile> {
            mod_file = ModFile {
                mod_id: fabric_mod_json.id,
                mod_version: fabric_mod_json.version,
                ..mod_file
            };
        }
    }

    mod_file
}

#[tauri::command]
pub async fn get_mod_file_list(path: String) -> Vec<ModFile> {
    let mut mod_file_list: Vec<ModFile> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let file_path = entry.unwrap().path();
        // println!("{:#?}", file_path);
        if file_path.is_file() && file_path.extension().unwrap() == "jar" {
            mod_file_list.push(get_mod_file(String::from(file_path.to_str().unwrap())).await);
        }
    }
    mod_file_list
}

#[derive(Serialize, Deserialize, Debug)]
// 序列化为 Json 后其值对应的就是名称
pub enum TargetKind {
    Local,
    Server,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub kind: TargetKind,
    pub location: String,
}

impl Target {
    pub fn from_str(value: String) -> Target {
        serde_json::from_str(&value).unwrap()
    }

    pub fn to_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    targets: Vec<Target>,
}

impl Config {
    fn default() -> Config {
        Config {
            targets: Vec::new(),
        }
    }

    pub fn get_target_list_json(&self) -> String {
        serde_json::to_string(&self.targets).unwrap()
    }

    pub fn get_target_json(&self, id: usize) -> String {
        self.targets.get(id).unwrap().to_str()
    }

    pub fn add_target(&mut self, target: Target) {
        self.targets.push(target);
    }

    pub fn from_string(value: String) -> Config {
        serde_json::from_str(&value).unwrap()
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn get_config() -> Config {
    let mut config = Config::default();

    use std::io::Write;
    match fs::File::open("amcm.json") {
        Ok(mut file) => {
            let mut json_str = String::new();
            file.read_to_string(&mut json_str)
                .expect("amcm.json read failed");
            config = Config::from_string(json_str);
        }
        Err(_) => {
            let mut file = fs::File::create("amcm.json").expect("amcm.json created failed");
            let json_str = config.to_string();
            file.write_all(json_str.as_bytes())
                .expect("default config wrote failed")
        }
    }
    config
}

pub fn save_config(config: Config) {
    use crate::json;
    json::save_json_str("amcm.json", config.to_string());
}
