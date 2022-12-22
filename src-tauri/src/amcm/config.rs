use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
// 序列化为 Json 后其值对应的就是名称
pub enum TargetKind {
    Local,
    Server,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Target {
    pub kind: TargetKind,
    pub location: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    targets: Vec<Target>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            targets: Vec::new(),
        }
    }

    pub fn targets(&self) -> Vec<Target> {
        self.targets.clone()
    }
    pub fn get_target(&self, index: usize) -> Result<Target, String> {
        if let Some(target) = self.targets.get(index) {
            Ok(target.clone())
        } else {
            Err(String::from("Index out of bound"))
        }
    }

    pub fn add_target(&mut self, target: Target) {
        self.targets.push(target);
    }
    pub fn del_target(&mut self, index: usize) {
        self.targets.remove(index);
    }
}

// pub fn get_config() -> Config {
//     let mut config = Config::default();

//     use std::io::Write;
//     if let Ok(mut file) = fs::File::open("amcm.json") {
//         let mut json_str = String::new();
//         file.read_to_string(&mut json_str)
//             .expect("amcm.json read failed");
//         config = Config::from_string(json_str);
//     } else {
//         let mut file = fs::File::create("amcm.json").expect("amcm.json created failed");
//         let json_str = config.to_string();
//         file.write_all(json_str.as_bytes())
//             .expect("default config wrote failed")
//     }
// }

// pub fn save_config(config: Config) {
//     use crate::json;
//     json::save_json_str("amcm.json", config.to_string());
// }
