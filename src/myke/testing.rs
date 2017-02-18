use std::env;
use std::path::Path;
use std::sync::Mutex;
use std::sync::mpsc;
use myke::action;
use myke::utils;
use capture;
use capture::Cappy;

#[cfg(test)]
pub struct TestTable<'a> {
    pub desc: &'a str,
    pub args: &'a str,
    pub expected: &'a str,
}

#[cfg(test)]
pub fn run_cli_test<'a>(dir: &str, tests: &[&'a TestTable]) {
    let (tx, rx) = mpsc::channel();
    let cappy = Box::new(Cappy {
        tx: Mutex::new(tx)
    });
    capture::set(cappy);
    for test in tests {
        chdir(dir, &|| {
            run(&test.args);
            for x in rx.try_iter() {
                println!("{}", x);
            }
        })
    }
    capture::void();
}

#[cfg(test)]
fn chdir(dir: &str, f: &Fn()) {
    if let Ok(old_cwd) = env::current_dir() {
        let path = Path::new(dir);
        if env::set_current_dir(path).is_ok() {
            f();
            env::set_current_dir(old_cwd).unwrap();
        }
    }
}

#[cfg(test)]
fn run(args: &str) {
    let argv = args.split(" ").map(|s| {s.to_owned()}).collect();
    let params_groups = utils::parse_param_groups(argv);
    action::action(params_groups);
}
