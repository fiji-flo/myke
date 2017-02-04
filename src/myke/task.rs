extern crate humantime;
extern crate yaml_rust;

use std::time::Duration;
use self::yaml_rust::Yaml;

pub struct Task {
    pub name: String,
    pub desc: String,
    pub cmd: String,
    pub before: String,
    pub after: String,
    pub shell: String,
    pub retry: u32,
    pub retry_delay: Duration,
}

impl Task {
    pub fn parse(name: String, yaml: &Yaml) -> Task {
        Task{
            name: name,
            desc: val!(yaml, "desc", ""),
            cmd: val!(yaml, "cmd", ""),
            before: val!(yaml, "before",""),
            after: val!(yaml, "after",""),
            shell: val!(yaml, "shell",""),
            retry: 0,
            retry_delay: Duration::from_secs(0)
        }
    }
}
