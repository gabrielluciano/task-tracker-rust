use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::task::TaskErrorCode::BadInput;

pub mod file_db;
pub mod memory_db;

#[derive(Debug)]
pub enum TaskErrorCode {
    BadInput,
    TaskDoesNotExist,
    TaskAlreadyDone,
    Generic,
}

#[derive(Debug)]
pub struct TaskError {
    code: TaskErrorCode,
    message: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    title: String,
    done: bool,
}

impl Task {
    pub fn new(id: u32, title: &str) -> Result<Task, TaskError> {
        if title.is_empty() {
            return Err(TaskError::err(BadInput, "title cannot be an empty string"));
        }
        Ok(Task {
            id,
            title: String::from(title),
            done: false,
        })
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

impl Display for TaskErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic => write!(f, "Generic"),
            Self::BadInput => write!(f, "BadInput"),
            Self::TaskDoesNotExist => write!(f, "TaskDoesNotExist"),
            Self::TaskAlreadyDone => write!(f, "TaskAlreadyDone"),
        }
    }
}

impl TaskError {
    fn err(code: TaskErrorCode, message: &str) -> TaskError {
        TaskError {
            code,
            message: String::from(message),
        }
    }
}

impl Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} Error] {}", self.code, self.message)
    }
}

impl Error for TaskError {}

pub trait TaskDatabase {
    fn new() -> Self;
    fn add(&mut self, title: &str) -> Result<Task, TaskError>;
    fn list(&self) -> Result<Vec<Task>, TaskError>;
    fn done(&mut self, id: u32) -> Result<(), TaskError>;
    fn remove(&mut self, id: u32) -> Result<(), TaskError>;
}
