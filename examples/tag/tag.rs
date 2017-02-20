myke_test_file!();

myke_test!(
    name tag;
    cd "examples/tag";
    "myke tagA/tag" => "tags1 tag";
    "myke tagC/tag" => "tags2 tag";
//    "myke --dry-run tag" => "(?s)tags1/tag: Will run.*tags2/tag: Will run";
    "myke tagA/tag" => "tags1 tag";
    "myke tagA/tag" => "(tags2){0}";
    "myke tagB/tag" => "tags1/tag";
    "myke tagB/tag" => "tags2/tag";
    "myke tagC/tag" => "(tags1){0}";
    "myke tagC/tag" => "tags2 tag";
);
