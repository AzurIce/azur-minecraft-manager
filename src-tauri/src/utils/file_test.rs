use super::file;

#[test]
fn get_file_sha1() {
    let path = "G:\\_MC\\__HMCL__\\.minecraft\\mods\\appleskin-fabric-mc1.19-2.4.1.jar";

    let sha1 = file::get_file_sha1(path);
    assert_eq!("7bd52695e82b1ddd1fdb3320154b68bf48dfff37", sha1);
    println!("{}", sha1);
}