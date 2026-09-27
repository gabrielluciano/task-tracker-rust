use crate::task::TaskDatabase;

use super::Task;
use super::TaskError;
use super::TaskErrorCode::TaskAlreadyDone;
use super::TaskErrorCode::TaskDoesNotExist;

pub struct MemoryDb {
    last_id: u32,
    tasks: Vec<Task>,
}

impl TaskDatabase for MemoryDb {
    fn new() -> Self {
        MemoryDb {
            last_id: 0,
            tasks: vec![],
        }
    }

    fn add(&mut self, title: &str) -> Result<Task, TaskError> {
        self.last_id += 1;
        let task = Task::new(self.last_id, title)?;
        let clone = task.clone();
        self.tasks.push(task); // In the json version this operation can fail, e.g. IO failures 
        Ok(clone)
    }

    fn list(&self) -> Result<Vec<Task>, TaskError> {
        Ok(self.tasks.clone()) // Again no error possible in the MemoryDb
    }

    fn done(&mut self, id: u32) -> Result<(), TaskError> {
        let task_opt = self.tasks.iter_mut().find(|task| task.id() == id);
        if let Some(task) = task_opt {
            let result = task.mark_done();
            if !result {
                Err(TaskError::err(TaskAlreadyDone, "the task was already done"))
            } else {
                Ok(())
            }
        } else {
            Err(TaskError::err(TaskDoesNotExist, "the task was not found"))
        }
    }

    fn remove(&mut self, id: u32) -> Result<(), TaskError> {
        let index_opt = self.tasks.iter().position(|task| task.id() == id);
        if let Some(index) = index_opt {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(TaskError::err(TaskDoesNotExist, "the task was not found"))
        }
    }
}
