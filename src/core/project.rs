extern crate core;
extern crate yaml_rust;
extern crate itertools;

use core::task::Task;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::PathBuf;
use self::yaml_rust::{Yaml,YamlLoader};

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
    pub tasks:     HashMap<String, Task>
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
            name: val!(doc, "project", ""),
            desc: val!(doc, "desc", ""),
            tags: extract_string_vec(&doc["tags"]),
            discover: extract_string_vec(&doc["discover"]),
            mixin: extract_string_vec(&doc["mixins"]),
            env: extract_string_map(&doc["env"]),
            env_files: extract_string_vec(&doc["env_files"]),
            tasks: extract_task_map(&doc["tasks"])
        };
        add_env_file(&p.src, &mut p.env_files);
        load_env(&p.env_files, &mut p.env);
        p.mixin();
        Ok(p)
    }

    fn mixin(&mut self) {
        for path in &self.mixin {
            if let Ok(p) = Project::from(&PathBuf::from(path)) {
                merge_vec(&mut self.tags, &p.tags);
                merge_vec(&mut self.discover, &p.discover);
                merge_vec(&mut self.env_files, &p.env_files);
                merge_env(&mut self.env, &p.env);
            }
        }
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}",
               self.name,
               self.tags.join(","),
               itertools::join(self.tasks.keys(), ","))
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

fn extract_task_map(yml: &Yaml) -> HashMap<String, Task> {
    let yaml_vec = yml.as_hash();
    match yaml_vec {
        Some(yaml_vec) => yaml_vec.iter().filter_map(|(k, v)| {
            match k.as_str() {
                Some(k) => Some((String::from(k), Task::parse(String::from(k), v))),
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
        merge_env(&mut env, &parse_env_file(env_file));
        let mut local = String::from((*env_file).clone());
        local.push_str(".local");
        merge_env(&mut env, &parse_env_file(&local));
    }
}

fn merge_env(env: &mut HashMap<String, String>, update: &HashMap<String, String>) {
    for (k,v) in update {
        if k == "PATH" {
            let path = match env.get(k) {
                Some(path) => prepend_path(path, v),
                None => v.clone()
            };
            env.insert(k.clone(), path);
        } else {
            env.insert(k.clone(), v.clone());
        }
    }
}

fn parse_env_file(path: &String) -> HashMap<String, String> {
    println!("trying to merge env from {}", &path);
    if let Ok(mut file) = File::open(path) {
        let mut env_str = String::new();
        match file.read_to_string(&mut env_str) {
            Ok(_) => {
                let env_vec = env_str.lines()
                    .map(|line| line.splitn(2, "="))
                    .map(|mut split| (split.next(), split.last()))
                    .filter_map(|(k, v)| {
                        match (k, v) {
                            (Some(k), Some(v)) => Some((String::from(k), String::from(v))),
                            _ => None
                        }
                    }).collect::<Vec<(String, String)>>();
                return HashMap::from_iter(env_vec);
            },
            Err(e) => {
                println!("Error reading {}: {}", &path, e);
            }
        }
    }
    HashMap::new()
}

fn merge_vec(target: &mut Vec<String>, from: &Vec<String>) {
    for x in from {
        if !target.into_iter().any(|y| { x == y}) {
            target.push(x.clone());
        }
    }
}

fn prepend_path(path: &String, update: &String) -> String {
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    let update_paths = env::split_paths(&update).collect::<Vec<_>>();
    paths.extend(update_paths);
    match env::join_paths(paths) {
        Ok(s) => s.into_string().unwrap_or(path.clone()),
        _ => path.clone()
    }
}
