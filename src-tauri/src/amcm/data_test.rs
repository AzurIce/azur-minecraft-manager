use super::data::{self, DATA};

use tokio::runtime::Runtime;

#[test]
fn update_mod_files() {
    let rt = Runtime::new().unwrap();
    let mod_files = 
        DATA.blocking_lock()
            .update_mod_files("G:\\_MC\\__HMCL__\\.minecraft\\mods");
    println!("{:#?}", mod_files);
}
