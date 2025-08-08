use std::env;

mod io;

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
            io::add(task);
            println!("Task added: {}", task);
        }
        "update" => {}
        "delete" => {}
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
