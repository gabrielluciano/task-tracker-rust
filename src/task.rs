pub mod memory_db;

pub struct Task {
    id: u32,
    title: String,
    done: bool,
}

impl Task {
    pub fn new(id: u32, title: &str) -> Task {
        Task {
            id,
            title: String::from(title),
            done: false,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn done(&self) -> bool {
        self.done
    }

    fn mark_done(&mut self) -> bool {
        let was = self.done;
        self.done = true;
        !was
    }
}

pub trait TaskDatabase {
    fn new() -> Self;
    fn add(&mut self, title: &str);
    fn list(&self) -> &[Task];
    fn done(&mut self, id: u32) -> bool;
    fn remove(&mut self, id: u32) -> bool;
}
