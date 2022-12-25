use crate::amcm::structures::ModFile;
use futures::executor::block_on;
use std::{fs, path::PathBuf, time};

#[test]
fn test() {
    let time_start = time::Instant::now();
    let mut file_list = Vec::new();
    for entry in fs::read_dir("G:/_MC/__HMCL__/.minecraft/mods").unwrap() {
        let file_path = entry.unwrap().path();
        if file_path.is_file() && file_path.extension().unwrap() == "jar" {
            // file_list.push(block_on(ModFile::of(&file_path)));
            file_list.push(modfile_of(&file_path));
        }
    }

    let time_end = time_start.elapsed();
    println!("File list: {:#?}", file_list);
    println!("Time cost: {:#?}", time_end);
}

#[test]
fn test_modfile_of() {
    {
        let time_start = time::Instant::now();
        use crate::CORE;
        let amcm = block_on(CORE.lock());
        let time_end = time_start.elapsed();
        println!("Lazy init - Time cost: {:#?}", time_end);
    }

    let time_start = time::Instant::now();
    // let modfile = block_on(ModFile::of(&PathBuf::from(
    //     "G:/_MC/__HMCL__/.minecraft/mods/appleskin-fabric-mc1.19-2.4.1.jar",
    // )));
    let modfile = modfile_of(&PathBuf::from(
        "G:/_MC/__HMCL__/.minecraft/mods/appleskin-fabric-mc1.19-2.4.1.jar",
    ));
    let time_end = time_start.elapsed();
    println!("Mod File: {:#?}", modfile);
    println!("Time cost: {:#?}", time_end);

    // let time_start = time::Instant::now();
    // let modfile = block_on(ModFile::of(&PathBuf::from(
    //     "G:/_MC/__HMCL__/.minecraft/mods/appleskin-fabric-mc1.19-2.4.1.jar",
    // )));
    // let time_end = time_start.elapsed();
    // println!("Mod File: {:#?}", modfile);
    // println!("Time cost: {:#?}", time_end);
}

fn modfile_of(path: &PathBuf) -> ModFile {
    use crate::utils::file;
    use crate::CORE;
    use crate::amcm::structures;
    
    let time_start = time::Instant::now();
    let filename = String::from(path.file_name().unwrap().to_str().unwrap());
    let time_end = time_start.elapsed();
    println!("Convert filename - Time cost: {:#?}", time_end);


    let sha1;
    {
        let time_start = time::Instant::now();
        sha1 = file::get_file_sha1(path.to_str().unwrap());
        let time_end = time_start.elapsed();
        println!("Get file sha1 - Time cost: {:#?}", time_end);
    }
    
    let enabled = path.extension().unwrap() == ".jar";
    let mut belong_state = structures::BelongState::Unknown;

    let mut amcm;
    {
        let time_start = time::Instant::now();
        amcm = block_on(CORE.lock());
        let time_end = time_start.elapsed();
        println!("Get amcm lock - Time cost: {:#?}", time_end);
    }

    {
        let time_start = time::Instant::now();
        if let Some(_) = amcm.data().get_project_id_from_hash(&sha1) {
            belong_state = structures::BelongState::Modrinth;
        }
        let time_end = time_start.elapsed();
        println!("get_project_id_from_hash - Time cost: {:#?}", time_end);
    }
    
    let time_end = time_start.elapsed();
    println!("Total - Time cost: {:#?}", time_end);

    ModFile {
        filename,
        sha1,
        enabled,
        belong_state,
    }
}