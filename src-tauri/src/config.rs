use serde::{Deserialize, Serialize};

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

    use std::fs;
    use std::io::{Read, Write};
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
