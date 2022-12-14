use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TargetKind {
    Local,
    Server,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    kind: TargetKind,
    location: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    targets: Vec<Target>,
}

impl Config {
    fn default() -> Config {
        Config { targets: Vec::new() }
    }

    pub fn get_target_list(&self) -> String {
        serde_json::to_string(&self.targets).unwrap()
    }

    fn from_string(value: String) -> Config {
        serde_json::from_str(&value).unwrap()
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug)]
struct Data {}

impl Data {
    fn default() -> Data {
        Data {}
    }
}


#[derive(Debug)]
pub struct Core {
    pub config: Config,
    data: Data,
}

impl Core {
    pub fn init() -> Core {
        let mut core = Core::default();

        core.reload_config();

        return core;
    }

    fn default() -> Core {
        Core {
            config: Config::default(),
            data: Data::default()
        }
    }

    fn reload_data(&self) {
        // TODO: Reload data
    }
    fn reload_config(&mut self) {
        self.config = Config::default();

        use std::fs;
        match fs::File::open("amcm.json") {
            Ok(mut file) => {
                let mut json_str = String::new();
                file.read_to_string(&mut json_str).expect("amcm.json read failed");
                self.config = Config::from_string(json_str);
            }
            Err(_) => {
                let mut file = fs::File::create("amcm.json").expect("amcm.json created failed");
                let json_str = self.config.to_string();
                file.write_all(json_str.as_bytes()).expect("default config wrote failed")
            }
        }
    }
}
