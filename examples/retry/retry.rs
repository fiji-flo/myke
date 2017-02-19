myke_test_file!();

myke_test!(
    name retry;
    cd "examples/retry";
    "myke" => r"\s*PROJECT\s*\|\s*TAGS\s*\|\s*TASKS\s*\n.*";
	  "myke --verbose retry" => r"(?s)retry/retry: Failed, Retrying 1/5 in 10ms.*Retrying 2/5.*Retrying 3/5.*Retrying 4/5.*Retrying 5/5.*retry/retry: Failed";
	  "myke retry" => r"(Retrying \d+){0}";
);
