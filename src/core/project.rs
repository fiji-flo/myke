extern crate yaml_rust;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use self::yaml_rust::{Yaml,YamlLoader};

#[derive(Debug)]
pub struct Project {
    pub src:       String,
    pub cwd:       String,
    pub name:      String,
    pub desc:      String,
    pub tags:      Vec<String>,
    pub discover:  Vec<String>,
    pub mixin:     Vec<String>,
    pub env:       HashMap<String, String>,
    pub env_files: Vec<String>,
    pub tasks:     HashMap<String, String>
}

impl Project {
    pub fn from(path: &PathBuf) -> io::Result<Project> {
        let src = get_file_path(path);
        let cwd = get_cwd(path);
        let mut file = try!(File::open(&src));
        let mut yml_str = String::new();
        try!(file.read_to_string(&mut yml_str));
        let docs = YamlLoader::load_from_str(yml_str.as_str()).unwrap();
        let doc = &docs[0];
        let mut p = Project{
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
        };
        add_env_file(&p.src, &mut p.env_files);
        load_env(&p.env_files, &mut p.env);
        Ok(p)
    }
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

fn get_file_path(path: &PathBuf) -> String {
    let is_file = path.is_file();

    let src = if is_file {
        String::from(path.to_str().unwrap())
    } else {
        String::from(path.join("myke.yml").to_str().unwrap())
    };
    src
}

fn get_cwd(path: &PathBuf) -> String {
    let is_file = path.is_file();

    let cwd = if is_file {
        String::from(path.parent().unwrap().to_str().unwrap())
    } else {
        String::from(path.to_str().unwrap())
    };
    cwd
}

fn add_env_file(src: &String, env_files: &mut Vec<String>) {
    let mut env = String::from(src.clone().trim_right_matches(".yml"));
    env.push_str(".env");
    env_files.push(env);
}

fn load_env(env_files: &Vec<String>,mut env: &mut HashMap<String, String>) {
    for ref env_file in env_files {
        merge_env(env_file, &mut env);
        let mut local = String::from((*env_file).clone());
        local.push_str(".local");
        merge_env(&local, &mut env);
    }
}

fn merge_env(path: &String, env: &mut HashMap<String, String>) {
    println!("trying to merge env from {}", &path);
    if let Ok(mut file) = File::open(path) {
        let mut env_str = String::new();
        match file.read_to_string(&mut env_str) {
            Ok(_) => {
                let env_update = env_str.lines()
                    .map(|line| line.splitn(2, "="))
                    .map(|mut split| (split.next(), split.last()))
                    .filter_map(|(k, v)| {
                        match (k, v) {
                            (Some(k), Some(v)) => Some((String::from(k), String::from(v))),
                            _ => None
                        }
                    }).collect::<Vec<(String, String)>>();
                for (k, v) in env_update {
                    env.insert(k, v);
                }
            },
            Err(e) => println!("Error reading {}: {}", &path, e)
        }
    }
}
