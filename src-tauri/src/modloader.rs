use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ModLoaderFile {
    pub id: String,
    pub version: String,
}