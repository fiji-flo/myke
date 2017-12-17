extern crate lazytable;
extern crate regex;
extern crate term_size;

use myke::execution;
use myke::query;
use myke::template;
use myke::utils;
use myke::workspace::Workspace;
use std::collections::VecDeque;
use std::env;
use std::path::Path;
#[cfg(not(test))]
use std::process;

const VERSION: &'static str = "0.9";
const USAGE: &'static str = "Usage:
  myke [--myke-options] [tag/]task [--task-options] ...

myke options:
  --file=     yml file to load (default: myke.yml)
  --dry-run   print tasks without running them
  --version   print myke version
  --template= template file to render
  --license   show open source licenses
  --verbose   show slightly more output

Help Options:
  --help      Show this help message
";

#[derive(Debug)]
enum Action {
    Run(String),
    DryRun(String),
    VerboseRun(String),
    Help,
    Version,
    Licenses,
    Template(String),
}

pub fn action(mut param_groups: utils::ParamGroups) {
    let a = parse(&param_groups.pop_front().unwrap());

    match a {
        Action::Help => out!("{}", USAGE),
        Action::Version => out!("{}", VERSION),
        Action::DryRun(file) => run(&file, param_groups, true, false),
        Action::VerboseRun(file) => run(&file, param_groups, false, true),
        Action::Run(file) => run(&file, param_groups, false, false),
        Action::Template(file) => template(&file),
        _ => {}
    }
}

fn template(path: &str) {
    let p = Path::new(&path);
    match template::template_file(p, env::vars()) {
        Ok(s) => out!("{}", s),
        Err(e) => {
            out!("[TEMPLATE_ERROR]: parsing error {}", e);
            #[cfg(not(test))]
            process::exit(1);
        }
    };
}

fn run(path: &str, mut param_groups: utils::ParamGroups, dry_run: bool, verbose: bool) {
    let workspace = Workspace::parse(path);
    let queries = query::parse_queries(&mut param_groups);
    if queries.is_empty() {
        list(&workspace);
    }
    for query in queries {
        if let Err(e) = execution::execute(&workspace, &query, dry_run, verbose) {
            if verbose {
                out!("[EXECUTION_ERROR]: {}", e);
            }
            #[cfg(not(test))]
            process::exit(1);
        }
    }
}

pub fn list(workspace: &Workspace) {
    #[cfg(not(test))]
    let width = term_size::dimensions()
        .and_then(|(w, _)| Some(w))
        .unwrap_or(1000);
    #[cfg(not(test))]
    let mut table = lazytable::Table::with_width(width);
    #[cfg(test)]
    let mut table = lazytable::Table::with_width(1000);
    table.set_title(vec![
        "PROJECT".to_owned(),
        "TAGS".to_owned(),
        "TASKS".to_owned(),
    ]);
    for p in &workspace.projects {
        if let Some((name, tags, tasks)) = p.get_columns() {
            if !tasks.is_empty() {
                table.add_row(vec![name, tags, tasks]);
            }
        }
    }
    out!("{}", table);
}

fn parse(options: &VecDeque<String>) -> Action {
    if options.has("--help") || options.has("-h") {
        return Action::Help;
    }
    if options.has("--licenses") {
        return Action::Licenses;
    }
    if options.has("--version") {
        return Action::Version;
    }
    if let Some(file) = options.get_by_prefix("--template=") {
        return Action::Template(file);
    }

    let file = options
        .get_by_prefix("--file=")
        .unwrap_or_else(|| String::from("myke.yml"));

    if options.has("--dry-run") || options.has("-n") {
        return Action::DryRun(file);
    }
    if options.has("--verbose") || options.has("-n") {
        return Action::VerboseRun(file);
    }
    Action::Run(file)
}

trait Parse {
    fn has(&self, m: &str) -> bool;
    fn get_by_prefix(&self, prefix: &str) -> Option<String>;
}

impl Parse for VecDeque<String> {
    fn has(&self, m: &str) -> bool {
        self.iter().any(|s| s == m)
    }
    fn get_by_prefix(&self, prefix: &str) -> Option<String> {
        self.iter()
            .find(|s| s.starts_with(prefix))
            .and_then(|s| Some(s.replace(prefix, "")))
    }
}
