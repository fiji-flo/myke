use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::iter::Iterator;
use std::path::{Path,PathBuf};
use std::time::Duration;

pub type ParamGroups = VecDeque<VecDeque<String>>;

pub fn add_env_file(src: &str, env_files: &mut Vec<String>) {
    let mut env = String::from(src.clone().trim_right_matches(".yml"));
    env.push_str(".env");
    env_files.push(env);
}

pub fn load_path(cwd: &str, path: &str) -> String {
    let mut paths = env::split_paths(path)
        .map(|p| {
            if !p.has_root() {
                return Path::new(cwd).join(p);
            }
            p
        }).collect::<Vec<_>>();
    paths.push(Path::new(cwd).join("bin"));
    match env::join_paths(paths) {
        Ok(s) => s.into_string().unwrap_or(path.to_owned()),
        _ => path.to_owned()
    }
}

pub fn load_env(env_files: &Vec<String>,mut env: &mut HashMap<String, String>) {
    for ref env_file in env_files {
        merge_env(&mut env, &parse_env_file(env_file), true);
        let mut local = String::from((*env_file).clone());
        local.push_str(".local");
        merge_env(&mut env, &parse_env_file(&local), true);
    }
}

pub fn merge_env(env: &mut HashMap<String, String>, update: &HashMap<String, String>, over: bool) {
    for (k,v) in update {
        if k == "PATH" {
            let path = match env.get(k) {
                Some(path) => {
                    if over {
                        prepend_path(path, v)
                    } else {
                        prepend_path(v, path)
                    }
                },
                None => v.clone()
            };
            env.insert(k.clone(), path);
        } else if over || !env.contains_key(k) {
            env.insert(k.clone(), v.clone());
        }
    }
}

pub fn parse_env_file(path: &str) -> HashMap<String, String> {
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
                out!("Error reading {}: {}", &path, e);
            }
        }
    }
    HashMap::new()
}

pub fn merge_vec<T: Eq + Clone>(target: &mut Vec<T>, from: &Vec<T>) {
    for x in from {
        if !target.into_iter().any(|y| { x == y}) {
            target.push(x.clone());
        }
    }
}

pub fn prepend_path(path: &str, update: &str) -> String {
    let paths = env::split_paths(&path).collect::<Vec<_>>();
    let mut update_paths = env::split_paths(&update).collect::<Vec<_>>();
    update_paths.extend(paths);
    match env::join_paths(update_paths) {
        Ok(s) => s.into_string().unwrap_or(path.to_owned()),
        _ => path.to_owned()
    }
}

pub fn parse_param_groups(args: Vec<String>) -> ParamGroups {
    let mut queries = VecDeque::new();
    let mut current = VecDeque::new();

    for arg in args {
        if !arg.starts_with("--") && !current.is_empty() {
            queries.push_back(current);
            current = VecDeque::new();
        }
        current.push_back(arg);
    }
    queries.push_back(current);
    queries
}

pub fn get_cwd(path: &PathBuf) -> String {
    let is_file = path.is_file();
    let full_path = match fs::canonicalize(path) {
        Ok(p) => p,
        _ => path.clone(),
    };

    let cwd = if is_file {
        String::from(full_path.parent().unwrap().to_str().unwrap())
    } else {
        String::from(full_path.to_str().unwrap())
    };
    cwd
}

pub fn add_to_path(update: &String) -> String {
    if let Some(path) = env::var_os("PATH") {
        match path.to_str() {
            Some(p) => prepend_path(update, p),
            None => update.clone()
        }
    } else {
        update.clone()
    }
}

pub fn parse_duration(duration_str: &str) -> Duration {
    if duration_str.ends_with("ms") {
        let ms = match duration_str.trim_right_matches("ms").parse::<u64>() {
            Ok(ms) => ms,
            _ => 1000
        };
        return Duration::from_millis(ms);
    }
    if duration_str.ends_with("ms") {
        let s = match duration_str.trim_right_matches("s").parse::<u64>() {
            Ok(s) => s,
            _ => 1
        };
        return Duration::from_secs(s);
    }
    return Duration::from_secs(1);
}
