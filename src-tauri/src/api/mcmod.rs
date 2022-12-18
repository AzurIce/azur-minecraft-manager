use std::fs;

#[tauri::command]
fn get_mod_filename_list(path: String) -> Vec<String> {
    let mut mod_filename_list: Vec<String> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let file_path = entry.unwrap().path();
        // println!("{:#?}", file_path);
        if file_path.is_file() && file_path.extension().unwrap() == "jar" {
            mod_filename_list.push(String::from(file_path.to_str().unwrap()));
        }
    }
    mod_filename_list
}