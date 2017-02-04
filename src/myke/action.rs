extern crate regex;

use myke::query;
use myke::utils;
use myke::template;
use myke::template::TemplateError;
use myke::workspace::Workspace;
use prettytable::Table;
use std::collections::VecDeque;
use std::env;
use std::path::Path;
use std::process;

const VERSION: &'static str = "0.9";
const USAGE: &'static str = "Usage:
  myke [--myke-options] [tag/]task [--task-options] ...

myke options:
      --file=     yml file to load (default: myke.yml)
  -n, --dry-run   print tasks without running them
      --version   print myke version
      --template= template file to render
      --license   show open source licenses

Help Options:
  -h, --help      Show this help message
";


#[derive(Debug)]
enum Action {
    Run(String),
    DryRun(String),
    Help,
    Version,
    Licenses,
    Template(String),
}

pub fn action(mut param_groups: utils::ParamGroups) {
    let a = parse(param_groups.pop_front().unwrap());
    println!("{:?}", a);

    match a {
        Action::Help => println!("{}", USAGE),
        Action::Version => println!("{}", VERSION),
        Action::DryRun(file) => run(file, param_groups, true),
        Action::Run(file) => run(file, param_groups, false),
        Action::Template(file) => template(file),
        _ => {}
    }
}

fn template(path: String) {
    let p = Path::new(&path);
    match template::template_file(&p , env::vars()) {
        Ok(s) => println!("{}", s),
        Err(TemplateError::Required) => {
            println!("[TEMPLATE_ERROR] missing required argument");
            process::exit(1);
        },
        Err(TemplateError::Unknown) =>  {
            println!("[TEMPLATE_ERROR: unknown error :/]");
            process::exit(1);
        },
    };
}

fn run(path: String, mut param_groups: utils::ParamGroups, dry: bool) {
    let workspace = Workspace::parse(&path);
    let queries = query::parse_queries(&mut param_groups);
    if queries.is_empty() {
        list(&workspace);
    }
    for query in queries {

    }

}

pub fn list(workspace: &Workspace) {
    let mut table = Table::new();
    table.add_row(row![bc->"project", bc->"tags", bc->"tasks"]);
    for p in &workspace.projects {
        let (name, tags, tasks) = p.get_columns();
        table.add_row(row![name, tags, tasks]);
    }
    table.printstd();
}

fn parse(options: VecDeque<String>) -> Action {
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
        return Action::Template(file)
    }

    let file = options.get_by_prefix("--file=").unwrap_or(String::from("myke.yml"));

    if options.has("--dry-run") || options.has("-n") {
        return Action::DryRun(file);
    }
    Action::Run(file)
}

trait Parse {
    fn has(&self, m: &str) -> bool;
    fn get_by_prefix(&self, prefix: &str) -> Option<String>;
}

impl Parse for VecDeque<String> {
    fn has(&self, m: &str) -> bool {
        self.iter().any(|s| {s == m})
    }
    fn get_by_prefix(&self, prefix: &str) -> Option<String> {
        self.iter().filter(|s| { s.starts_with(prefix)}).next().and_then(
            |s| { Some(s.replace(prefix, "")) }
        )
    }
}
