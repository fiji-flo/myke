#[macro_use]
extern crate prettytable;
mod core;

use core::action;
use core::utils;
use std::env;

fn main() {
    let param_groups = utils::parse_param_groups(env::args().collect::<Vec<_>>());
    action::action(param_groups);
}
