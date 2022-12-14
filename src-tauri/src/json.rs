use std::{fs::File, io::{Read, Write}};

#[tauri::command]
pub fn get_json_str(path: &str) -> String {
    if let Ok(mut file) = File::open(path) {
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Read json failed");
        return buf;
    } else {
        return String::from("{}");
    }
}

#[tauri::command]
pub fn save_json_str(path: &str, json: String) {
    let mut file = File::create(path).expect("Create json failed");

    file.write(json.as_bytes()).expect("Write json failed");
}