myke_test_file!();

myke_test!(
    name list;
    cd "examples";
    "myke" => r"\s*PROJECT\s*\|\s*TAGS\s*\|\s*TASKS\s*\n.*";
    "myke" => r"(?m)^\s*env\s*\|\s*\|\s*file_custom, file_custom_local, file_default, file_default_local, path, yml\s*$";
    "myke" => r"(?m)^\s*hooks\s*\|\s*\|\s*after, before, error\s*$";
    "myke" => r"(?m)^\s*mixin\s*\|\s*\|\s*path, task1, task2, task3, task4\s*$";
    "myke" => r"(?m)^\s*retry\s*\|\s*\|\s*retry\s*$";
    "myke" => r"(?m)^\s*tags1\s*\|\s*tagA, tagB\s*\|\s*tag\s*$";
    "myke" => r"(?m)^\s*tags2\s*\|\s*tagB, tagC\s*\|\s*tag\s*$";
    "myke" => r"(?m)^\s*template\s*\|\s*\|\s*args, file\s*$";
);
