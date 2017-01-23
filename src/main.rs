#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
mod core;

use core::workspace::Workspace;
use prettytable::Table;

fn main() {
    let matches = clap_app!(myke =>
                            (version: "0.9")
                            (about: "myke - your friendly task runner")
                            (@arg FILE: -f --file +takes_value "`yml` file to load")
                            (@arg TEMPLATE: --template "render template `tpl-file` (will not run any command)")
                            (@arg LICENSE: --license "show license")
                            (@arg LOGLEVEL: --loglevel "log level, one of debug|`info`|warn|error|fatal")
    ).get_matches();
    if let Some(yml) = matches.value_of("FILE") {
        let workspace = Workspace::parse(yml);
        list(&workspace);
    }
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
