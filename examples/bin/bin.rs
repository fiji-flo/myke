myke_test_file!();

myke_test!(
    name bin;
    cd "examples/bin";
	  "myke path" => r"PATH is [^:]+.*bin[/\\]bin";
);
