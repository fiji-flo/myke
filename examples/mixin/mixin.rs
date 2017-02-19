myke_test_file!();

myke_test!(
    name mixin;
    cd "examples/mixin";
    "myke" => r"\s*PROJECT\s*\|\s*TAGS\s*\|\s*TASKS\s*\n.*";
    "myke task1" => "parent says value_parent_1";
    "myke task2" => "(?s)parent says value_child_2.*?child says value_child_2";
    "myke task3" => "child says value_child_3";
    "myke path" => "PATH is [^:]+.*/mixin/path_child:[^:]+.*/mixin/bin:[^:]+.*/mixin/parent/path_parent:[^:]+.*/mixin/parent/bin";
);
