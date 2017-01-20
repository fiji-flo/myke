extern crate yaml_rust;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use self::yaml_rust::{Yaml,YamlLoader};

#[derive(Debug)]
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

fn extract_string_vec(yml: &Yaml) -> Vec<String> {
    let yaml_vec = yml.as_vec();
    match yaml_vec {
        Some(yaml_vec) => yaml_vec.iter()
            .filter_map(|x| x.as_str())
            .map(|x| String::from(x))
            .collect(),
        _ => Vec::new()
    }
}

fn extract_string_map(yml: &Yaml) -> HashMap<String, String> {
    let yaml_vec = yml.as_hash();
    match yaml_vec {
        Some(yaml_vec) => yaml_vec.iter().filter_map(|(k, v)| {
            match (k.as_str(), v.as_str()) {
                (Some(k), Some(v)) => Some((String::from(k), String::from(v))),
                _ => None
            }
        }).collect(),
        _ => HashMap::new()
    }
}

fn get_file_path(path: &str) -> String {
    let path = Path::new(path);
    let is_file = path.is_file();

    let src = if is_file {
        String::from(path.to_str().unwrap())
    } else {
        String::from(path.join("myke.yml").to_str().unwrap())
    };
    src
}

fn get_cwd(path: &str) -> String {
    let path = Path::new(path);
    let is_file = path.is_file();

    let cwd = if is_file {
        String::from(path.parent().unwrap().to_str().unwrap())
    } else {
        String::from(path.to_str().unwrap())
    };
    cwd
}

impl Project {
    pub fn parse(path: &str) -> io::Result<Project> {
        let src = get_file_path(path);
        let cwd = get_cwd(path);
        let mut file = try!(File::open(&src));
        let mut yml_str = String::new();
        try!(file.read_to_string(&mut yml_str));
        let docs = YamlLoader::load_from_str(yml_str.as_str()).unwrap();
        let doc = &docs[0];
        Ok(Project{
            src: src,
            cwd: cwd,
            name: String::from(doc["project"].as_str().unwrap_or("")),
            desc: String::from(doc["desc"].as_str().unwrap_or("")),
            tags: extract_string_vec(&doc["tags"]),
            discover: extract_string_vec(&doc["discover"]),
            mixin: extract_string_vec(&doc["mixins"]),
            env: extract_string_map(&doc["env"]),
            env_files: extract_string_vec(&doc["env_files"]),
            tasks: extract_string_map(&doc["tasks"])
        })
    }
}
