extern crate glob;
use std::io::prelude::*;
use std::process::Command;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use glob::glob;

fn main() {
    let tests = glob("examples/**/*.rs").expect("Failed to read glob pattern").filter_map(|entry| {
        match entry {
            Ok(path) =>  Some(copy(&path)),
            _ => None
        }
    }).map(|mut file_name| { convert(file_name)}).collect::<Vec<String>>();
    make_mod(&tests);
}

fn copy(path: &Path) -> String{
    let file_name = path.file_name().unwrap();
    let to = Path::new("src/myke/tests/").join(file_name);
    fs::copy(path, to);
    String::from(file_name.to_str().unwrap())
}

fn make_mod(tests: &Vec<String>) {
    let path = Path::new("src/myke/tests/mod.rs");
    let mut f = File::create(&path).unwrap();
    for test in tests {
        write!(f, "mod {};\n", test);
    }

}

fn convert(mut file_name: String) -> String {
    let cut = file_name.len() - 3;
    file_name.truncate(cut);
    file_name
}
