#[macro_use]
extern crate clap;
mod core;

use core::workspace::Workspace;

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
        for p in workspace.projects {
            println!("{}", p);
        }
    }
}
