myke_test_file!();

myke_test!(
    name template;
    cd "examples/template";
    "myke --file=myke.yml" => r"\s*PROJECT\s*\|\s*TAGS\s*\|\s*TASKS\s*\n.*";
    "myke --file=myke.yml args --from=a --to=b" => r"from=a to=b";
    "myke --file=myke.yml args --from=a" => r"from=a to=something_to";
);
