use myke::project::Project;
use myke::query::Query;
use myke::task::Task;
use myke::workspace::Workspace;
use std::string::String;

struct Execution<'a> {
    workspace: &'a Workspace,
    query: &'a Query,
    project: &'a Project,
    task: &'a Task,
}

impl <'a>Execution<'a> {
    pub fn execute(&'a self) -> Result<(), String> {
        Ok(())
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
        e.execute();
    }
    Ok(())
}
