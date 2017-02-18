myke_test_file!();

myke_test!(
    name tag;
    cd "examples";
    "myke tagA/tag" => r"tags1 tag\n.*";
    "myke tagC/tag" => r"tags2 tag\n.*";
);
