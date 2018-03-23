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

use clap::ArgMatches;

#[derive(Debug)]
enum Action {
    Run(String),
    DryRun(String),
    VerboseRun(String),
    Help,
    Template(String),
}

pub fn action(args: &ArgMatches, queries: utils::ParamGroups) {
    let a = parse(args);

    match a {
        Action::Help => out!("{}", args.usage()),
        Action::DryRun(file) => run(&file, queries, true, false),
        Action::VerboseRun(file) => run(&file, queries, false, true),
        Action::Run(file) => run(&file, queries, false, false),
        Action::Template(file) => template(&file),
    }
}

fn template(path: &str) {
    let p = Path::new(&path);
    match template::template_file(p, env::vars()) {
        Ok(s) => out!("{}", s),
        Err(e) => {
            error!("[TEMPLATE_ERROR]: parsing error {}", e);
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
            error!("error running command: {}", e);
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

fn parse(args: &ArgMatches) -> Action {
    if args.is_present("help") {
        return Action::Help;
    }
    if let Some(file) = args.value_of("template") {
        return Action::Template(file.into());
    }

    let file = args.value_of("file").unwrap_or_else(|| "myke.yml").into();

    if args.is_present("dry-run") {
        return Action::DryRun(file);
    }
    if args.is_present("verbose") {
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
