extern crate itertools;
extern crate yaml_rust;

use self::yaml_rust::{Yaml, YamlLoader};
use myke::task::Task;
use myke::utils::*;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct Project {
    pub src: String,
    pub cwd: String,
    pub name: String,
    pub desc: String,
    pub tags: Vec<String>,
    pub discover: Vec<String>,
    pub mixin: Vec<String>,
    pub env: HashMap<String, String>,
    pub env_files: Vec<String>,
    pub tasks: HashMap<String, Task>,
}

impl Project {
    pub fn from(path: &PathBuf) -> io::Result<Project> {
        let src = get_file_path(path);
        let cwd = get_cwd(path);
        let mut file = File::open(&src)?;
        let mut yml_str = String::new();
        file.read_to_string(&mut yml_str)?;
        match YamlLoader::load_from_str(&yml_str) {
            Ok(docs) => {
                let doc = &docs[0];
                let mut p = Project {
                    src,
                    cwd,
                    name: val!(doc, "project", ""),
                    desc: val!(doc, "desc", ""),
                    tags: extract_string_vec(&doc["tags"]),
                    discover: extract_string_vec(&doc["discover"]),
                    mixin: extract_string_vec(&doc["mixin"]),
                    env: extract_string_map(&doc["env"]),
                    env_files: extract_string_vec(&doc["env_files"]),
                    tasks: extract_task_map(&doc["tasks"]),
                };
                add_env_file(&p.src, &mut p.env_files);
                load_env(&p.env_files, &mut p.env);
                update_path(&p.cwd, &mut p.env);
                p.mixin();
                Ok(p)
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("ERROR parsing {}: {}", src, e),
            )),
        }
    }

    fn mixin(&mut self) {
        for path in &self.mixin {
            if let Ok(p) = Project::from(&PathBuf::from(&self.cwd).join(path)) {
                update_tasks(&mut self.tasks, &p.tasks);
                merge_vec(&mut self.tags, &p.tags);
                merge_vec(&mut self.discover, &p.discover);
                merge_vec(&mut self.env_files, &p.env_files);
                merge_env(&mut self.env, &p.env, false);
            }
        }
    }

    pub fn get_columns(&self) -> Option<(String, String, String)> {
        if self.tasks.is_empty() {
            return None;
        }
        Some((
            self.name.clone(),
            itertools::join(itertools::sorted(&self.tags), ", "),
            itertools::join(
                itertools::sorted(self.tasks.keys().filter(|x| !x.starts_with('_'))),
                ", ",
            ),
        ))
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}\t{}",
            self.name,
            self.tags.join(","),
            itertools::join(self.tasks.keys(), ",")
        )
    }
}

fn get_file_path(path: &PathBuf) -> String {
    if path.is_file() {
        String::from(path.to_str().unwrap())
    } else {
        String::from(path.join("myke.yml").to_str().unwrap())
    }
}

fn extract_string_vec(yml: &Yaml) -> Vec<String> {
    yml.as_vec()
        .map(|yaml_vec| {
            yaml_vec
                .iter()
                .filter_map(|x| x.as_str())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

fn extract_string_map(yml: &Yaml) -> HashMap<String, String> {
    yml.as_hash()
        .map(|yaml_vec| {
            yaml_vec
                .iter()
                .filter_map(|(k, v)| match (k.as_str(), v.as_str()) {
                    (Some(k), Some(v)) => Some((String::from(k), String::from(v))),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default()
}

fn extract_task_map(yml: &Yaml) -> HashMap<String, Task> {
    yml.as_hash()
        .map(|yaml_vec| {
            yaml_vec
                .iter()
                .filter_map(|(k, v)| match k.as_str() {
                    Some(k) => Some((String::from(k), Task::parse(String::from(k), v))),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn update_tasks(base: &mut HashMap<String, Task>, update: &HashMap<String, Task>) {
    for (k, v) in update {
        if let Some(t) = base.get_mut(k) {
            t.update(v);
        }
        if !base.contains_key(k) {
            base.insert(k.clone(), v.clone());
        }
    }
}
