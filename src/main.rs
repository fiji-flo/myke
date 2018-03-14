#[cfg(test)]
#[macro_use]
extern crate capture;
extern crate colored;
extern crate gtmpl;
mod myke;

use myke::action;
use myke::utils;
use std::env;

fn main() {
    let param_groups = utils::parse_param_groups(env::args().collect::<Vec<_>>());
    action::action(param_groups);
}
