extern crate glob;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use glob::glob;

fn main() {
    let tests = glob("examples/**/*.rs")
        .expect("Failed to read glob pattern")
        .filter_map(|entry| match entry {
            Ok(path) => copy(&path),
            _ => None,
        })
        .map(|file_name| convert(file_name))
        .collect::<Vec<String>>();
    make_mod(&tests);
}

fn copy(path: &Path) -> Option<String> {
    let file_name = path.file_name().unwrap();
    let to = Path::new("src/myke/tests/").join(file_name);
    match fs::copy(path, to) {
        Ok(_) => Some(String::from(file_name.to_str().unwrap())),
        _ => None,
    }
}

fn make_mod(tests: &Vec<String>) {
    let path = Path::new("src/myke/tests/mod.rs");
    let mut f = File::create(&path).unwrap();
    for test in tests {
        let _ = write!(f, "mod {};\n", test);
    }

}

fn convert(mut file_name: String) -> String {
    let cut = file_name.len() - 3;
    file_name.truncate(cut);
    file_name
}
