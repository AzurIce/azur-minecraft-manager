use crate::config::{Mod, ModEnv, ModSource};
use ferinth::{Error, Ferinth};

pub async fn get_mod_from_hash(sha1: String) -> Result<Mod, Error> {
    let sha1 = sha1.to_lowercase();

    let modrinth = Ferinth::default();
    let version = modrinth.get_version_from_hash(&sha1).await?;

    get_mod(version.project_id).await
}

/*
pub struct Mod {
    source: ModSource,
    id: i32,
    mod_name: String, // title field of Modrinth project
    mod_description: String,
    mod_env: ModEnv,
}
*/

use ferinth::structures::project_structs::ProjectSupportRange::Unsupported;
pub async fn get_mod(project_id: String) -> Result<Mod, Error> {
    let modrinth = Ferinth::default();
    let project = modrinth.get_project(&project_id).await?;

    let mod_info = Mod::default();
    let mut mod_env = ModEnv::Unknown;

    if project.client_side != Unsupported && project.server_side != Unsupported {
        mod_env = ModEnv::Both
    } else if project.client_side != Unsupported {
        mod_env = ModEnv::Client
    } else if project.server_side != Unsupported {
        mod_env = ModEnv::Server
    }

    Ok(Mod {
        source: ModSource::Modrinth,
        id: project_id,
        mod_name: project.title,
        mod_description: project.description,
        mod_env,
        ..mod_info
    })
}
