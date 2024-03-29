use std::fs::File;
use std::io::Read;

pub fn read_source_file(path: &str) -> String {
    println!("Reading source file: {}", path);
    let mut file = File::open("tests/fl/".to_owned() + path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
