#[cfg(test)]
#[macro_use]
extern crate capture;
extern crate clap;
extern crate colored;
extern crate gtmpl;
extern crate yaml_rust;
mod myke;

use std::env;

use myke::action;
use myke::utils;

fn main() {
    let matches = utils::parse_args(env::args_os());

    let queries = matches
        .values_of("tasks")
        .map(utils::parse_task_groups)
        .unwrap_or_default();
    action::action(&matches, queries);
}
