use crate::amcm::data::DATA;

use super::mod_source;

#[test]
fn add_mod_source() {
    let project_id = "EsAfCjCV".to_owned();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(mod_source::add_mod_source(project_id));
    println!("{:#?}", res);
}

#[test]
fn add_mod_source_from_local_mod_file() {
    let project_id = "EsAfCjCV".to_owned();
    let version_id = "pIFVoWvG".to_owned();
    let mod_file_hash = "7bd52695e82b1ddd1fdb3320154b68bf48dfff37".to_owned();

    let rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(
        DATA.blocking_lock()
            .update_mod_files("G:\\_MC\\__HMCL__\\.minecraft\\mods");
    // );
    rt.block_on(mod_source::add_mod_source_from_local_mod_file(
        project_id,
        version_id,
        mod_file_hash,
    ));
}
