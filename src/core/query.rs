extern crate core;
extern crate regex;

use core::project::Project;
use core::task::Task;
use self::regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;

pub struct Query {
    task: String,
    tags: Vec<String>,
    params: HashMap<String, String>,
}

pub struct QueryMatch<'a> {
    project: &'a Project,
    task: &'a Task
}

impl Query {
    pub fn parse(mut raw: VecDeque<String>) -> Query {
        let cmd = raw.pop_front().unwrap_or(String::new());
        let mut cmds: Vec<&str> = cmd.split("/").collect();
        let task = String::from(cmds.pop().unwrap_or(""));
        let tags = cmds.iter().map(|t| { String::from(*t) }).collect();

        let mut params = HashMap::new();

        let param_re = Regex::new(r"--\(.+\)=\(.*\)").unwrap();
        for rparam in raw {
            if let Some(cap) = param_re.captures(rparam.as_str()) {
                if let (Some(k), Some(v)) = (cap.get(1), cap.get(2)) {
                    params.insert(String::from(k.as_str()), String::from(v.as_str()));
                }
            }
        }

        Query {
            task: task,
            tags: tags,
            params: params
        }
    }
}

pub fn parse_queries() -> Vec<VecDeque<String>> {
    let args = env::args();
    let mut queries = Vec::new();
    let mut current = VecDeque::new();

    for arg in args {
        if !arg.starts_with("--") && !current.is_empty() {
            queries.push(current);
            current = VecDeque::new();
        }
        current.push_back(String::from(arg));
    }
    queries.push(current);
    for query in &queries {
        println!("{:?}", query);
    }
    queries
}
