use std::path::PathBuf;
use myke::project::Project;
use myke::utils;
use std::string::String;

pub struct Workspace {
    pub cwd: String,
    pub projects: Vec<Project>,
}

impl Workspace {
    pub fn parse(path: &str) -> Workspace {
        let mut projects = Vec::new();
        let cwd = utils::get_cwd(&PathBuf::from(path));
        let src = utils::get_file_path(&PathBuf::from(path));
        Workspace::traverse(&src.unwrap_or_else(|| cwd.clone()), None, &mut projects);
        Workspace {
            cwd: cwd,
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
                Workspace::traverse(&p.cwd, Some(include), projects)
            }
            projects.push(p);
        }
    }
}
