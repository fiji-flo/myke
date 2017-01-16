extern crate yaml_rust;

use std::collections::HashMap;
use std::fs::File;
use std::vec;
use yaml_rust::{YamlLoader, YamlEmitter};

pub struct Project {
    src:       String,
    cwd:       String,
    name:      String,
    desc:      String,
    tags:      Vec<String>,
    discover:  Vec<String>,
    mixin:     Vec<String>,
    env:       HashMap<String, String>,
    env_files: Vec<String>,
    tasks:     HashMap<String, String>
}

impl Project {
    parse(path: String) -> Project {
        let file = try!(File::open(path));
        let mut yml_str = try!(file.read_to_string(&file));
        let yml = YamlLoader::load_from_str(yml_str);
    }
}
