use serde_json;
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::task;

pub fn add(task: &task::Task) {
    // Read tasks from file first
    let mut tasks = get_tasks_from_file();

    tasks.tasks.push((*task).clone());
    tasks.last_id = task.id;

    write(&tasks);
}

pub fn get_last_id() -> u32 {
    let file_content = fs::read_to_string("tasks.json");

    match file_content {
        Ok(content) => {
            match content.trim() {
                "" => return 0, // If the file is empty, return 0
                tasks_str => {
                    let tasks: task::TaskList =
                        serde_json::from_str(&tasks_str).expect("Unable to parse tasks");
                    tasks.last_id
                }
            }
        }
        Err(_) => 0, // If the file doesn't exist or is empty, return 0
    }
}

pub fn delete(id: u32) {
    // Read tasks from file first
    let mut tasks = get_tasks_from_file();

    tasks.tasks.retain(|task| task.id != id);

    write(&tasks);
}

// Reads the tasks from the file
pub fn get_tasks_from_file() -> task::TaskList {
    let file_content = fs::read_to_string("tasks.json");
    match file_content {
        Ok(content) => {
            if content.trim().is_empty() {
                task::TaskList {
                    tasks: Vec::new(),
                    last_id: 0,
                }
            } else {
                serde_json::from_str(&content).unwrap_or(task::TaskList {
                    tasks: Vec::new(),
                    last_id: 0,
                })
            }
        }
        Err(_) => task::TaskList {
            tasks: Vec::new(),
            last_id: 0,
        },
    }
}

pub fn list_tasks(status_filter: Option<&str>) -> Vec<task::Task> {
    let tasks = get_tasks_from_file().tasks;

    match status_filter {
        Some("done") => tasks
            .into_iter()
            .filter(|t| t.status == task::Status::Done)
            .collect(),
        Some("in-progress") => tasks
            .into_iter()
            .filter(|t| t.status == task::Status::InProgress)
            .collect(),
        Some("todo") => tasks
            .into_iter()
            .filter(|t| t.status == task::Status::Todo)
            .collect(),
        _ => tasks, // If no filter is provided, return all tasks
    }
}

fn write(tasks: &task::TaskList) {
    let mut file = File::create("tasks.json").expect("Unable to open tasks file");
    file.write_all(serde_json::to_string(tasks).unwrap().as_bytes())
        .expect("Unable to write tasks to file");
    file.flush().expect("Unable to flush tasks file");
}

pub fn update(task: &task::Task) {
    let mut tasks = get_tasks_from_file();
    if let Some(existing_task) = tasks.tasks.iter_mut().find(|t| t.id == task.id) {
        existing_task.description = task.description.clone();
        existing_task.status = task.status.clone();
        existing_task.updated_at = task.updated_at;
    } else {
        println!("Task with ID {} not found.", task.id);
        return;
    }
    write(&tasks);
}

pub fn get_task_by_id(id: u32) -> Option<task::Task> {
    let tasks = get_tasks_from_file();
    tasks.tasks.into_iter().find(|t| t.id == id)
}
