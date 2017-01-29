#[macro_use]
extern crate prettytable;
mod core;

use core::query;
use core::workspace::Workspace;
use prettytable::Table;

fn main() {
    let queries = query::parse_queries();
    let file = match queries.get(0)
        .and_then(|p| { p.get(1)})
        .and_then(|fps| {
            let mut fp = fps.splitn(2, "=");
            match (fp.next(), fp.last()) {
                (Some("--file"), Some(file)) => Some(file),
                _ => None
            }
        }) {
            Some(file) => file,
            _ => "myke.yml"
        };
    let workspace = Workspace::parse(file);
    list(&workspace);
}

fn list(workspace: &Workspace) {
    let mut table = Table::new();
    table.add_row(row![bc->"project", bc->"tags", bc->"tasks"]);
    for p in &workspace.projects {
        let (name, tags, tasks) = p.get_columns();
        table.add_row(row![name, tags, tasks]);
    }
    table.printstd();
}
