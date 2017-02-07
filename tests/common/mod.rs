extern crate myke;

use std::env;
use std::path::Path;

pub struct TestTable {
    desc: String,
    args: String,
    expected: String,
}

pub fn run_cli_test(dir: &str, test: &[TestTable]) {
}

pub fn chdir(dir: &str, f: &Fn()) {
    if let Ok(old_cwd) = env::current_dir() {
        let path = Path::new(dir);
        if env::set_current_dir(path).is_ok() {
            f();
            env::set_current_dir(old_cwd).unwrap();
        }
    }
}
