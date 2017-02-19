extern crate humantime;
extern crate yaml_rust;

use std::time::Duration;
use myke::utils::parse_duration;
use self::yaml_rust::Yaml;

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub desc: Option<String>,
    pub cmd: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub shell: Option<String>,
    pub error: Option<String>,
    pub retry: u32,
    pub retry_delay: (Duration, String),
}

impl Task {
    pub fn parse(name: String, yaml: &Yaml) -> Task {
        let dur = val!(yaml, "retry_delay", "1s");
        Task{
            name: name,
            desc: val_opt!(yaml, "desc"),
            cmd: val_opt!(yaml, "cmd"),
            before: val_opt!(yaml, "before"),
            after: val_opt!(yaml, "after"),
            shell: val_opt!(yaml, "shell"),
            error: val_opt!(yaml, "error"),
            retry: yaml["retry"].as_i64().unwrap_or(0) as u32,
            retry_delay: (parse_duration(&dur), dur),
        }
    }

    pub fn update(&mut self, update: &Task) {
        if self.name == update.name {
            update_task!(self update desc);
            update_task!(self update cmd);
            update_task!(self update before);
            update_task!(self update after);
            update_task!(self update shell);
            update_task!(self update error);
        }
    }
}
