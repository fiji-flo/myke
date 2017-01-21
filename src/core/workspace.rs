extern crate core;
use std::path::{Path,PathBuf};
use core::project::Project;
use std::string::String;

#[derive(Debug)]
pub struct Workspace {
    cwd:      String,
    projects: Vec<Project>,
}

impl Workspace {
    pub fn parse(cwd: &str) -> Workspace {
        let mut projects = Vec::new();
        let mut cwd = Path::new(cwd);
        if cwd.is_file() {
            cwd = cwd.parent().unwrap();
        }
        let cwd = cwd.to_str().unwrap();
        Workspace::traverse(cwd, None, &mut projects);
        Workspace{
            cwd: String::from(cwd),
            projects: projects,
        }
    }

    fn traverse(cwd: &str, path: Option<&str>, projects: &mut Vec<Project>) {
        let mut src = PathBuf::from(cwd);
        if let Some(path) = path {
           src.push(path);
        }
        if let Ok(p) = Project::from(&src) {
            for include in &p.discover {
                println!("including {}", include);
                Workspace::traverse(p.cwd.as_str(), Some(include.as_str()), projects)
            }
            projects.push(p);
        }
    }
}
