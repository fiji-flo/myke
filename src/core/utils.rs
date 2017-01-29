use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

pub fn add_env_file(src: &str, env_files: &mut Vec<String>) {
    let mut env = String::from(src.clone().trim_right_matches(".yml"));
    env.push_str(".env");
    env_files.push(env);
}

pub fn load_env(env_files: &Vec<String>,mut env: &mut HashMap<String, String>) {
    for ref env_file in env_files {
        merge_env(&mut env, &parse_env_file(env_file));
        let mut local = String::from((*env_file).clone());
        local.push_str(".local");
        merge_env(&mut env, &parse_env_file(&local));
    }
}

pub fn merge_env(env: &mut HashMap<String, String>, update: &HashMap<String, String>) {
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

pub fn parse_env_file(path: &str) -> HashMap<String, String> {
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

pub fn merge_vec(target: &mut Vec<String>, from: &Vec<String>) {
    for x in from {
        if !target.into_iter().any(|y| { x == y}) {
            target.push(x.clone());
        }
    }
}

pub fn prepend_path(path: &str, update: &str) -> String {
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    let update_paths = env::split_paths(&update).collect::<Vec<_>>();
    paths.extend(update_paths);
    match env::join_paths(paths) {
        Ok(s) => s.into_string().unwrap_or(path.to_owned()),
        _ => path.to_owned()
    }
}
