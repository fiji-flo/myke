extern crate itertools;
extern crate regex;

use self::regex::Regex;
use capture;
use capture::Cappy;
use myke::action;
use myke::utils;
use std::env;
use std::path::Path;
use std::sync::Mutex;

#[cfg(test)]
pub struct TestTable<'a> {
    pub desc: &'a str,
    pub args: &'a str,
    pub expected: &'a str,
}

#[cfg(test)]
pub fn run_cli_test<'a>(dir: &str, tests: &[TestTable]) {
    let buf = Mutex::new(vec![]);
    let cappy = Box::new(Cappy { buf });
    capture::set(cappy);
    for test in tests {
        chdir(dir, &|| {
            run(&test.args);
            let out = capture::dump().unwrap_or("".to_owned());
            let re = Regex::new(test.expected).unwrap();
            assert!(
                re.is_match(&out),
                "\nexpected:\n{}\n\ngot:\n{}",
                test.expected,
                out
            );
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
    let argv = args.split(" ").map(|s| s.to_owned());

    let matches = utils::parse_args(argv);
    let queries = matches
        .values_of("tasks")
        .map(utils::parse_task_groups)
        .unwrap_or_default();
    action::action(&matches, queries);
}

#[cfg(test)]
#[macro_export]
macro_rules! myke_test_file {
    () => {
        #[cfg(test)]
        use myke::testing;
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! myke_test {
    (
        $(before $before:block)*
        $(after $after:block)*
        name $name:ident;
        cd $dir:expr;
        $(
            $cmd:expr => $should:expr;
        )*
    ) => {
        #[test]
        fn $name() {
            $($before)*
            let tt = vec!(
                $(testing::TestTable{
                    desc: "",
                    args: $cmd,
                    expected: $should
                },)*
            );
            testing::run_cli_test($dir, &tt);
            $($after)*
        }
    }
}
