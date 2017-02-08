extern crate myke;
mod common;
use common::TestTable;
use myke::myke::utils;

#[test]
fn working() {
    let mut a = vec!(1,2,3);
    let b = vec!(1,2,3,4);
    utils::merge_vec(&mut a, &b);
}

#[test]
fn help() {
    let tt = TestTable{
        desc: "",
        args: "myke --help",
        expected: ""
    };
    common::run_cli_test("examples", &vec!(&tt));
}
