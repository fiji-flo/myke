myke_test_file!();

myke_test!(
    name hooks;
    cd "examples/hooks";
    "myke" => r"\s*PROJECT\s*\|\s*TAGS\s*\|\s*TASKS\s*\n.*";
    "myke before" => r"running before";
    "myke after" => r"running after";
    "myke error" => r"there was an error.*";
);
