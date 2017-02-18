myke_test_file!();

myke_test!(
    name list;
    cd "examples";
    "myke" => r"\s*PROJECT\s*\|\s*TAGS\s*\|\s*TASKS\s*\n.*";
);
