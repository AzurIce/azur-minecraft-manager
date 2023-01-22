// use crate::utils::file::get_file_sha1;
// use crate::amcm::core::CORE;
// use serde::{Deserialize, Serialize};
// use std::path::{PathBuf, Path};


pub mod mod_file;

// Source
// pub mod mod_source;
// use mod_source::ModSource;
// enum Source {
//     Mod(ModSource),
// 	// ResourcePackSource,
// 	// ShaderPackSource,
// 	// ...
// }

pub mod mods_json;

// //----- ModFileBelong -----//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct ModFileBelong {
//     pub version_id: String,
//     pub project_id: String,
// }

// impl ModFileBelong {
//     pub fn new(version_id: String, project_id: String) -> ModFileBelong {
//         ModFileBelong {
//             version_id,
//             project_id,
//         }
//     }
// }

// //----- ModFile -----//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum BelongState {
//     Unknown,
//     Modrinth,
// }

