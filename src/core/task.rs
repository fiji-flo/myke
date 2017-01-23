extern crate humantime;
extern crate yaml_rust;

use std::time::Duration;
use self::yaml_rust::Yaml;

pub struct Task {
    name: String,
    desc: String,
    before: String,
    after: String,
    shell: String,
    retry: u32,
    retry_delay: Duration,
}

impl Task {
    pub fn parse(name: String, yaml: &Yaml) -> Task {
        Task{
            name: name,
            desc: val!(yaml, "desc", ""),
            before: val!(yaml, "before",""),
            after: val!(yaml, "after",""),
            shell: val!(yaml, "shell",""),
            retry: 0,
            retry_delay: Duration::from_secs(0)
        }
    }
}
