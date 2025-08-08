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
        "update" => {}
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
        "mark-in-progress" => {}
        "mark-done" => {}
        "list" => {}
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
