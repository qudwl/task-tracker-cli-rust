use serde_json;
use std::fs;
use std::io::Write;

use crate::task;

pub fn add(task: &task::Task) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("tasks.json")
        .expect("Unable to open tasks file");

    let mut tasks: task::TaskList = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(_) => task::TaskList {
            tasks: Vec::new(),
            last_id: 0,
        },
    };

    tasks.tasks.push((*task).clone());
    tasks.last_id = task.id;

    file.write_all(serde_json::to_string(&tasks).unwrap().as_bytes())
        .expect("Unable to write tasks to file");
    file.flush().expect("Unable to flush tasks file");
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
