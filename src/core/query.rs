extern crate core;

use core::project::Project;
use core::task::Task;
use std::collections::HashMap;
use std::env;

pub struct Query {
    raw: String,
    task: String,
    tags: Vec<String>,
    params: HashMap<String, String>,
}

pub struct QueryMatch<'a> {
    project: &'a Project,
    task: &'a Task
}

//impl Query {
//    pub fn parse::(raw: &str[]) -> Query {
//    }
//
//}

pub fn parse_queries() -> Vec<Vec<String>> {
    let args = env::args();
    let mut queries = Vec::new();
    let mut current = Vec::new();

    for arg in args {
        if !arg.starts_with("--") && !current.is_empty() {
            queries.push(current);
            current = Vec::new();
        }
        current.push(String::from(arg));
    }
    queries.push(current);
    for query in &queries {
        println!("{:?}", query);
    }
    queries
}
