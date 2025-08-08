use chrono::{DateTime, Local, Utc};
use std::env;
use std::time::SystemTime;

mod io;
mod task;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task to add.");
                help();
                return;
            }

            let task = &args[2];
            add_task(task);
            println!("Task added: {}", task);
        }
        "update" => {
            if args.len() < 4 {
                println!("Please provide a task ID and description to update.");
                help();
                return;
            }

            let id: u32 = match args[2].parse() {
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid task ID: {}", args[2]);
                    return;
                }
            };

            let description = &args[3];
            update(id, Some(description), None);
        }
        "delete" => {
            if args.len() < 3 {
                println!("Please provide a task ID to delete.");
                help();
                return;
            }

            let id: u32 = match args[2].parse() {
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid task ID: {}", args[2]);
                    return;
                }
            };

            io::delete(id);
            println!("Task with ID {} deleted.", id);
        }
        "mark-in-progress" => {
            if args.len() < 3 {
                println!("Please provide a task ID to mark as in progress.");
                help();
                return;
            }

            let id: u32 = match args[2].parse() {
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid task ID: {}", args[2]);
                    return;
                }
            };

            update(id, None, Some(task::Status::InProgress));
        }
        "mark-done" => {
            if args.len() < 3 {
                println!("Please provide a task ID to mark as in progress.");
                help();
                return;
            }

            let id: u32 = match args[2].parse() {
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid task ID: {}", args[2]);
                    return;
                }
            };

            update(id, None, Some(task::Status::Done));
        }
        "list" => {
            if args.len() == 2 {
                list_tasks(None);
            } else if args.len() == 3 {
                let status_filter = &args[2];
                match status_filter.as_str() {
                    "done" => list_tasks(Some("done")),
                    "in-progress" => list_tasks(Some("in-progress")),
                    "todo" => list_tasks(Some("todo")),
                    _ => {
                        println!("Invalid status filter: {}", status_filter);
                        help();
                        return;
                    }
                }
            } else {
                println!("Invalid usage of list command.");
                help();
                return;
            }
        }
        "help" => help(),
        _ => {
            println!("Unknown command: {}", args[1]);
            help();
        }
    }
}

// Prints the help message for the CLI
fn help() {
    println!("Usage: task-cli [options]");
    println!("Options:");
    println!("  add \"[task]\"      Add a new task");
    println!("  update \"[id]\"     Update an existing task");
    println!("  delete \"[id]\"     Delete a task");
    println!("");
    println!("  mark-in-progress \"[id]\"    Mark a task as in progress");
    println!("  mark-done \"[id]\"           Mark a task as done");
    println!("  list    List all tasks");
    println!("  list \"[done|todo|in-progress]\"  List tasks by status");
    println!("");
    println!("  help      Show this help message");
}

fn add_task(description: &str) {
    let last_id = io::get_last_id();
    let task = task::Task {
        id: last_id + 1,
        description: description.to_string(),
        status: task::Status::Todo,
        created_at: SystemTime::now(),
        updated_at: SystemTime::now(),
    };

    io::add(&task);
    println!("Task added with ID: {}", task.id);
}

fn list_tasks(status_filter: Option<&str>) {
    let tasks = io::list_tasks(status_filter);
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks {
            let created_utc: DateTime<Utc> = task.created_at.into();
            let created_local: DateTime<Local> = created_utc.with_timezone(&Local);
            let updated_utc: DateTime<Utc> = task.updated_at.into();
            let updated_local: DateTime<Local> = updated_utc.with_timezone(&Local);

            println!(
                "ID: {}, Description: {}, Status: {:?}, Created At: {}, Updated At: {}",
                task.id,
                task.description,
                task.status,
                created_local.format("%Y-%m-%d %H:%M:%S %Z"),
                updated_local.format("%Y-%m-%d %H:%M:%S %Z")
            );
        }
    }
}

fn update(id: u32, description: Option<&str>, status: Option<task::Status>) {
    let check_list = io::get_task_by_id(id);
    if let Some(mut task) = check_list {
        if let Some(desc) = description {
            task.description = desc.to_string();
        }
        if let Some(stat) = status {
            task.status = stat;
        }
        task.updated_at = SystemTime::now();
        io::update(&task);
        println!("Task with ID {} updated.", id);
    } else {
        println!("Task with ID {} not found.", id);
    }
}
