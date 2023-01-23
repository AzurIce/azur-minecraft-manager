use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModsJson {
    pub remote_mods: Vec<String>
}