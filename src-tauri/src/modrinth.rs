use crate::mcmod::{Mod, ModEnv, ModSource};
use ferinth::{Error, Ferinth};

use ferinth::structures::project_structs::ProjectSupportRange::Unsupported;
pub async fn get_mod(project_id: String) -> Result<Mod, Error> {
    let modrinth = Ferinth::default();
    let project = modrinth.get_project(&project_id).await?;

    let mut mod_env = ModEnv::Unknown;

    if project.client_side != Unsupported && project.server_side != Unsupported {
        mod_env = ModEnv::Both
    } else if project.client_side != Unsupported {
        mod_env = ModEnv::Client
    } else if project.server_side != Unsupported {
        mod_env = ModEnv::Server
    }

    Ok(Mod {
        source: ModSource::Modrinth(project_id),
        name: project.title,
        desc: project.description,
        env: mod_env,
    })
}
