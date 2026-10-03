use std::fs;

use super::Task;
use super::TaskError;
use super::TaskErrorCode::TaskDoesNotExist;
use crate::task::TaskDatabase;
use crate::task::TaskErrorCode::Generic;

const FILENAME: &str = "tasks.json";

pub struct FileDb {}

fn get_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    if fs::exists(FILENAME)? {
        let tasks_serial = fs::read_to_string(FILENAME)?;
        let tasks: Vec<Task> = serde_json::from_str(&tasks_serial)?;
        Ok(tasks)
    } else {
        Ok(vec![])
    }
}

fn write_tasks(tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    Ok(fs::write(FILENAME, serde_json::to_string(tasks)?)?)
}

fn with_tasks<T>(handler: impl Fn(&mut Vec<Task>) -> Result<T, TaskError>) -> Result<T, TaskError> {
    match get_tasks() {
        Ok(mut tasks) => {
            let result = handler(&mut tasks)?;
            match write_tasks(&tasks) {
                Ok(_) => Ok(result),
                Err(e) => Err(TaskError::err(Generic, &e.to_string())),
            }
        }
        Err(e) => Err(TaskError::err(Generic, &e.to_string())),
    }
}

impl TaskDatabase for FileDb {
    fn new() -> Self {
        FileDb {}
    }

    fn add(&mut self, title: &str) -> Result<Task, TaskError> {
        with_tasks(|tasks| {
            let last_id = tasks.iter().map(|t| t.id()).max().unwrap_or(0);
            let id = last_id + 1;
            let task = Task::new(id, title)?;
            tasks.push(task.clone());
            Ok(task)
        })
    }

    fn list(&self) -> Result<Vec<Task>, TaskError> {
        match get_tasks() {
            Ok(tasks) => Ok(tasks),
            Err(e) => Err(TaskError::err(Generic, &e.to_string())),
        }
    }

    fn done(&mut self, id: u32) -> Result<(), TaskError> {
        with_tasks(|tasks| {
            let task_opt = tasks.iter_mut().find(|task| task.id() == id);
            if let Some(task) = task_opt {
                task.mark_done()?;
                Ok(())
            } else {
                Err(TaskError::err(TaskDoesNotExist, "the task was not found"))
            }
        })
    }

    fn remove(&mut self, id: u32) -> Result<(), TaskError> {
        with_tasks(|tasks| {
            let index_opt = tasks.iter().position(|task| task.id() == id);
            if let Some(index) = index_opt {
                tasks.remove(index);
                Ok(())
            } else {
                Err(TaskError::err(TaskDoesNotExist, "the task was not found"))
            }
        })
    }
}
