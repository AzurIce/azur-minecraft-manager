use super::file;

#[test]
fn get_file_sha1() {
    let path = "G:\\_MC\\__HMCL__\\.minecraft\\mods\\appleskin-fabric-mc1.19-2.4.1.jar";
    let path = "G:\\_MC\\__HMCL__\\.minecraft\\mods\\amcm_iris-mc1.19.3-1.5.1.jar";

    let sha1 = file::get_file_sha1(path);
    // assert_eq!("7bd52695e82b1ddd1fdb3320154b68bf48dfff37", sha1);
    assert_eq!("bfd36e8fb2577990e23612ed73d27b6ae2fffca2", sha1);
    println!("{}", sha1);
}