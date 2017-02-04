use myke::project::Project;
use myke::query::Query;
use myke::task::Task;
use myke::template;
use myke::workspace::Workspace;
use std::process::Command;
use std::string::String;
use std::time::Instant;

struct Execution<'a> {
    workspace: &'a Workspace,
    query: &'a Query,
    project: &'a Project,
    task: &'a Task,
}

impl <'a>Execution<'a> {
    pub fn execute(&'a self) -> Option<()> {
        let now = Instant::now();
        self.retry();
        println!("{}/{} Took: {}", self.project.name, self.task.name, now.elapsed().as_secs());
        Some(())
    }

    fn retry(&'a self) -> Option<()> {
        for _ in 0..self.task.retry {
            if let None = self.execute_task() {
            } else {
                return Some(());
            }
        }
        None

    }

    fn execute_task(&'a self) -> Option<()> {
        self.execute_cmd(&self.task.before)
            .and_then(|_| { self.execute_cmd(&self.task.cmd) })
            .and_then(|_| { self.execute_cmd(&self.task.after) })
    }

    fn execute_cmd(&'a self, cmd: &str) -> Option<()> {
        let cmd = match template::template_str(cmd, &self.project.env) {
            Ok(s) => s,
            _ => String::from(cmd)
        };
        let mut command = Command::new("sh");
        for (k, v) in &self.project.env {
            command.env(k, v);
        }
        let status = command
            .arg("-exc")
            .env("MYKE_PROJECT", &self.project.name)
            .env("MYKE_TASK", &self.task.name)
            .env("MYKE_CWD", &self.project.cwd)
            .args(&cmd.split(" ").collect::<Vec<_>>())
            .current_dir(&self.project.cwd)
            .status()
            .expect(format!("failed to execute {}", self.task.cmd).as_str());
        if status.success() {
            Some(())
        } else {
            None
        }
    }
}

pub fn execute(w: &Workspace, q: &Query) -> Result<(), String> {
    let matches = q.search(w);
    if matches.is_empty() {
        return Err(format!("no task matched {}", q.task));
    }
    for m in matches {
        let e = Execution{
            workspace: w,
            query: q,
            project: &m.project,
            task: &m.task,
        };
        if let None = e.execute() {
            return Err(String::from("DOOM"));
        }
    }
    Ok(())
}
