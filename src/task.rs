use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    pub last_id: u32,
}
