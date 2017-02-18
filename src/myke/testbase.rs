use myke::testing::TestTable;
use myke::utils;
#[cfg(test)]
use myke::testing;

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
        args: "myke",
        expected: ""
    };
    testing::run_cli_test("examples", &vec!(&tt));
}
