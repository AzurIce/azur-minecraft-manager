use super::cache;

use futures::executor;
use tokio::runtime::Runtime;

// #[test]
// fn test_ferinth() {
//     // let rt = Runtime::new().unwrap();
//     let modrinth = ferinth::Ferinth::default();
//     let res = modrinth
//         .get_version_from_hash("7bd52695e82b1ddd1fdb3320154b68bf48dfff37");
//     println!("{:#?}", res);
// }

#[test]
fn update_version_from_hash() {
    // let rt = Runtime::new().unwrap();
    let res = 
        cache::CACHE
            .blocking_lock()
            .update_version_from_hash("7bd52695e82b1ddd1fdb3320154b68bf48dfff37");
    println!("{:#?}", res);
}

#[test]
fn get_version_from_hash() {
    // appleskin-fabric-mc1.19-2.4.1.jar
    // let rt = Runtime::new().unwrap();
    let res = cache::CACHE
        .blocking_lock()
        .get_version_from_hash("7bd52695e82b1ddd1fdb3320154b68bf48dfff37").unwrap();
    assert_eq!("pIFVoWvG", res.id);
    println!("{:#?}", res);
}

#[test]
// fn get_version_id_from_hash() {
//     // appleskin-fabric-mc1.19-2.4.1.jar
//     let rt = Runtime::new().unwrap();
//     let res = rt.block_on(cache::CACHE
//         .blocking_lock()
//         .get_version_id_from_hash("7bd52695e82b1ddd1fdb3320154b68bf48dfff37"));
//     println!("{:#?}", res);
// }

#[test]
fn get_versions_from_hashes() {
    // let rt = Runtime::new().unwrap();
    let res = cache::CACHE.blocking_lock().get_versions_from_hashes(vec![
        "7bd52695e82b1ddd1fdb3320154b68bf48dfff37".to_owned(),
        "9c6173d2c38306fd8f87d33e259d33fecb5e1dea".to_owned(),
        "17438ba70d88752f73d814405927450c81d035b9".to_owned(),
    ]);
    println!("{:#?}", res);
}
