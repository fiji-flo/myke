extern crate itertools;
extern crate glob;
extern crate regex;

use self::itertools::join;
use self::glob::Pattern;
use self::regex::Regex;
use myke::project::Project;
use myke::task::Task;
use myke::utils::ParamGroups;
use myke::workspace::Workspace;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Query {
    pub raw: String,
    pub task: String,
    pub tags: Vec<String>,
    pub params: HashMap<String, String>,
}

pub struct QueryMatch<'a> {
    pub project: &'a Project,
    pub task: &'a Task
}

impl Query {
    pub fn parse(mut rparams: &mut VecDeque<String>) -> Query {
        let raw = join(rparams.clone(), " ");
        let cmd = rparams.pop_front().unwrap_or(String::new());
        let mut cmds: Vec<&str> = cmd.split("/").collect();
        let task = String::from(cmds.pop().unwrap_or(""));
        let tags = cmds.iter().map(|t| { String::from(*t) }).collect();

        let mut params = HashMap::new();

        let param_re = Regex::new(r"--\(.+\)=\(.*\)").unwrap();
        for rparam in rparams {
            if let Some(cap) = param_re.captures(rparam.as_str()) {
                if let (Some(k), Some(v)) = (cap.get(1), cap.get(2)) {
                    params.insert(String::from(k.as_str()), String::from(v.as_str()));
                }
            }
        }

        Query {
            raw: raw,
            task: task,
            tags: tags,
            params: params
        }
    }

    pub fn search<'a>(&self, w: &'a Workspace) -> Vec<QueryMatch> {
        let mut matches = Vec::new();
        for p in &w.projects {
            for (_, t) in &p.tasks {
                if let Some(m) = self.matches(&p, &t) {
                    matches.push(m);
                }
            }
        }
        vec!()
    }

    fn matches<'a>(&'a self, p: &'a Project, t: &'a Task) -> Option<QueryMatch> {
        for tag in &self.tags {
            let pattern = Pattern::new(tag.as_str()).unwrap();
            let mut hit = pattern.matches(p.name.as_str());
            for t in &p.tags {
                hit = hit || pattern.matches(t.as_str());
            }
            if !hit {
                return None;
            }
        }
        if Pattern::new(self.task.as_str()).unwrap().matches(t.name.as_str()) {
            return Some(QueryMatch{ project: p, task: t });
        }
        None
    }
}

pub fn parse_queries(param_groups: &mut ParamGroups) -> Vec<Query> {
    param_groups.iter_mut().map(|mut q| { Query::parse(q) }).collect()
}
