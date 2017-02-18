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
fn tag_a() {
    let tt = TestTable{
        desc: "",
        args: "myke tagA/tag",
        expected: r"tags1 tag\n.*"
    };
    testing::run_cli_test("examples", &vec!(&tt));
}

#[test]
fn tag_c() {
    let tt = TestTable{
        desc: "",
        args: "myke tagC/tag",
        expected: r"tags2 tag\n.*"
    };
    testing::run_cli_test("examples", &vec!(&tt));
}
