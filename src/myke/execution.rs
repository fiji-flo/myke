use myke::project::Project;
use myke::query::Query;
use myke::task::Task;
use myke::workspace::Workspace;
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
            if let None = self.executeTask() {
            } else {
                return Some(());
            }
        }
        None

    }

    fn executeTask(&'a self) -> Option<()> {
        self.executeCmd(&self.task.before)
            .and_then(|_| { self.executeCmd(&self.task.cmd) })
            .and_then(|_| { self.executeCmd(&self.task.after) })
    }

    fn executeCmd(&'a self, cmd: &str) -> Option<()> {
        Some(())
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
