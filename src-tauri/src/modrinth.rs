// use ferinth::structures::version_structs::Version;
// use ferinth::{Error, Ferinth};


// pub async fn get_version_from_hash(sha1: &str, refresh: bool) -> Result<Version, Error> {
//     let mut db: Database<Version> = Database::new("modrinth-hash-version.jdb");
//     if refresh || !ModrinthAPI::api().hash_to_version.contains_key(sha1) {
//         let modrinth = Ferinth::default();
//         let version = modrinth.get_version_from_hash(sha1).await?;
//         let mut api = ModrinthAPI::api();
//         api.hash_to_version.insert(String::from(sha1), version.clone());
//         Ok(version)
//     } else {
//         Ok(ModrinthAPI::api().hash_to_version.get(sha1).unwrap().clone())
//     }
// }

// use ferinth::structures::project_structs::ProjectSupportRange::Unsupported;
// use jasondb::Database;
// use once_cell::sync::OnceCell;
// pub async fn get_mod(project_id: String, refresh: bool) -> Result<Mod, Error> {
//     let modrinth = Ferinth::default();
//     let project = modrinth.get_project(&project_id).await?;

//     let mut mod_env = ModEnv::Unknown;

//     if project.client_side != Unsupported && project.server_side != Unsupported {
//         mod_env = ModEnv::Both
//     } else if project.client_side != Unsupported {
//         mod_env = ModEnv::Client
//     } else if project.server_side != Unsupported {
//         mod_env = ModEnv::Server
//     }

//     Ok(Mod {
//         source: ModSource::Modrinth(project_id),
//         name: project.title,
//         desc: project.description,
//         env: mod_env,
//     })
// }
