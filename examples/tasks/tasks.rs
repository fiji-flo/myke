myke_test_file!();

myke_test!(
    name tasks;
    cd "examples/tasks";
    "myke" => r"\s*PROJECT\s*\|\s*TAGS\s*\|\s*TASKS\s*\n.*";
    "myke" => r"\s*tasks\s*\|\s*\|\s*visible\s*\n.*";
    "myke visible" => "visible";
    "myke _hidden" => "hidden";
);
