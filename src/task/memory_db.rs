use crate::task::TaskDatabase;

use super::Task;

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

    fn add(&mut self, title: &str) {
        self.last_id += 1;
        self.tasks.push(Task::new(self.last_id, title));
    }

    fn list(&self) -> &[Task] {
        &self.tasks
    }

    fn done(&mut self, id: u32) -> bool {
        let task_opt = self.tasks.iter_mut().find(|task| task.id() == id);
        if let Some(task) = task_opt {
            return task.mark_done();
        }
        false
    }

    fn remove(&mut self, id: u32) -> bool {
        let index_opt = self.tasks.iter().position(|task| task.id() == id);
        if let Some(index) = index_opt {
            self.tasks.remove(index);
            return true;
        }
        false
    }
}
