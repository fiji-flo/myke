extern crate glob;
extern crate checksums;
extern crate tempdir;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
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
    if hash(&path.to_path_buf()) != hash(&to) && fs::copy(path, to).is_err() {
       None
    } else {
        Some(String::from(file_name.to_str().unwrap()))
    }
}

fn make_mod(tests: &Vec<String>) {
    if let Ok(dir) = tempdir::TempDir::new("tests") {
        let path = dir.path().join("mod.rs");
        let mut f = File::create(&path).unwrap();
        for test in tests {
            let _ = write!(f, "mod {};\n", test);
        }
        assert!(copy(&path).is_some());
    }
}

fn convert(mut file_name: String) -> String {
    let cut = file_name.len() - 3;
    file_name.truncate(cut);
    file_name
}

fn hash(file: &PathBuf) -> String {
    if !file.exists() {
        "".to_string()
    } else {
        checksums::hash_file(file, checksums::Algorithm::MD5)
    }
}
