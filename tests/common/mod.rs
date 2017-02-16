extern crate myke;

use std::env;
use std::path::Path;


pub struct TestTable<'a> {
    pub desc: &'a str,
    pub args: &'a str,
    pub expected: &'a str,
}

pub fn run_cli_test<'a>(dir: &str, tests: &[&'a TestTable]) {
    for test in tests {
        chdir(dir, &|| {
            run(&test.args);
        })
    }
}

fn chdir(dir: &str, f: &Fn()) {
    if let Ok(old_cwd) = env::current_dir() {
        let path = Path::new(dir);
        if env::set_current_dir(path).is_ok() {
            f();
            env::set_current_dir(old_cwd).unwrap();
        }
    }
}

fn run(args: &str) {
    let argv = args.split(" ").map(|s| {s.to_owned()}).collect();
    let params_groups = myke::myke::utils::parse_param_groups(argv);
    myke::myke::action::action(params_groups);
}
