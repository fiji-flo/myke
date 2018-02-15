use myke::project::Project;
use myke::query::Query;
use myke::task::Task;
use myke::template;
use myke::utils;
use myke::workspace::Workspace;
use std::collections::HashMap;
use std::env;
use std::env::current_exe;
#[cfg(windows)]
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use std::string::String;
use std::thread::sleep;
use std::time::Instant;

struct Execution<'a> {
    query: &'a Query,
    project: &'a Project,
    task: &'a Task,
    dry_run: bool,
    verbose: bool,
}

impl<'a> Execution<'a> {
    pub fn execute(&'a self) -> Option<()> {
        let name = format!("{}/{}", self.project.name, self.task.name);
        if self.dry_run {
            out!("{}: Will run", name);
        }
        let now = Instant::now();
        let status = self.retry();
        let took = now.elapsed();
        let took = format!(
            "{}.{:>0w$}s",
            took.as_secs(),
            took.subsec_nanos() / 1000,
            w = 6
        );
        match status {
            Some(_) => out!("{}: Completed, Took: {}", name, took),
            _ => out!("{}: Failed, Took: {}", name, took),
        }
        status
    }

    fn retry(&'a self) -> Option<()> {
        for i in 0..(self.task.retry + 1) {
            if self.execute_task().is_some() {
                return Some(());
            }
            if let Some(ref err) = self.task.error {
                if !err.is_empty() {
                    out!("{}", err);
                }
            }
            sleep(self.task.retry_delay.0);
            if i < self.task.retry && self.verbose {
                out!(
                    "{}/{}: Failed, Retrying {}/{} in {}ms",
                    self.project.name,
                    self.task.name,
                    i + 1,
                    self.task.retry,
                    self.task.retry_delay.1
                );
            }
        }
        None
    }

    fn execute_task(&'a self) -> Option<()> {
        self.execute_cmd(&self.task.before)
            .and_then(|_| self.execute_cmd(&self.task.cmd))
            .and_then(|_| self.execute_cmd(&self.task.after))
    }

    #[cfg(windows)]
    fn shell() -> Command {
        let cmd_exe =
            env::var_os("ComSpec").unwrap_or(OsString::from(r"C:\Windows\system32\cmd.exe"));
        let mut cmd = Command::new(cmd_exe);
        cmd.arg("/c");
        cmd
    }

    #[cfg(unix)]
    fn shell() -> Command {
        let mut cmd = Command::new("sh");
        cmd.arg("-exc");
        cmd
    }

    fn execute_cmd(&'a self, cmd: &Option<String>) -> Option<()> {
        if let Some(ref cmd) = *cmd {
            if cmd == "" {
                return Some(());
            }
            let mut merged_env: HashMap<String, String> = env::vars().collect();
            utils::merge_env(&mut merged_env, &self.project.env, false);
            let mut cmd = match template::template_str(cmd, &merged_env, &self.query.params) {
                Ok(s) => s,
                Err(e) => {
                    out!("{} in {}", e, cmd);
                    return None;
                }
            };
            if let Some(ref shell) = self.task.shell {
                if shell != "" {
                    cmd = format!("{} {}", shell, cmd);
                }
            }
            let mut command = Execution::shell();
            for (k, v) in &merged_env {
                if k == "PATH" {
                    command.env(k, utils::add_to_path(v));
                } else {
                    command.env(k, v);
                }
            }
            command
                .env(
                    "myke",
                    current_exe().unwrap_or_else(|_| PathBuf::from("myke")),
                )
                .env("MYKE_PROJECT", &self.project.name)
                .env("MYKE_TASK", &self.task.name)
                .env("MYKE_CWD", &self.project.cwd)
                .arg(&cmd)
                .current_dir(&self.project.cwd);
            let status = run(&mut command, &format!("failed to execute {}", cmd));
            if status.success() {
                Some(())
            } else {
                None
            }
        } else {
            Some(())
        }
    }
}

#[cfg(not(test))]
fn run(command: &mut Command, error_msg: &str) -> ExitStatus {
    command.status().expect(error_msg)
}

#[cfg(test)]
fn run(command: &mut Command, error_msg: &str) -> ExitStatus {
    let output = command.output().expect(error_msg);
    out!("{}", String::from_utf8_lossy(&output.stdout));
    output.status
}

pub fn execute(
    workspace: &Workspace,
    query: &Query,
    dry_run: bool,
    verbose: bool,
) -> Result<(), String> {
    let matches = query.search(workspace);
    if matches.is_empty() {
        return Err(format!("no task matched {}", query.task));
    }
    for (project, task) in matches {
        let e = Execution {
            query,
            project,
            task,
            dry_run,
            verbose,
        };
        if e.execute().is_none() {
            return Err(String::from("Something went wrong :/"));
        }
    }
    Ok(())
}
