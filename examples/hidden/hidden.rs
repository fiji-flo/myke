myke_test_file!();

myke_test!(
    name hidden;
    cd "examples/hidden";
    "myke" => r"^\s*PROJECT\s*\|\s*TAGS\s*\|\s*TASKS\s*\n[-+]*\s*something\s*\|\s*\|\s*hello\s*\n$";
);
