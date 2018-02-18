myke_test_file!();

myke_test!(
    before { use std::env; env::set_var("_MYKE_ENV_VAR", "foobar2000"); }
    after { use std:: env; env::remove_var("_MYKE_ENV_VAR"); }
    name env;
    cd "examples/env";
    "myke" => r"\s*PROJECT\s*\|\s*TAGS\s*\|\s*TASKS\s*\n.*";
	  "myke yml" => r"envvar from yml is value_from_yml";
	  "myke file_default" => r"envvar from myke.env is value_from_myke.env";
	  "myke file_default_local" => r"envvar from myke.env.local is value_from_myke.env.local";
	  "myke file_custom" => r"envvar from test.env is value_from_test.env";
	  "myke file_custom_local" => r"envvar from test.env.local is value_from_test.env.local";
    // myke-rs expandes PATH elements
	  "myke path" => r"PATH is [^:;]+.*path_from_myke.env.local[:;][^:;]+.*path_from_myke.env[:;][^:;]+.*path_from_test.env.local[:;][^:;]+.*path_from_test.env[:;][^:;]+.*path_from_yml[:;][^:]+.*env[/\\]bin";
	  "myke system" => r"foobar2000";
);
