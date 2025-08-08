use std::fs::File;
use std::io::Write;

pub fn add(task: &str) {
    let mut f = File::create("tasks.json").expect("Unable to create file");

    f.write_all(task.as_bytes()).expect("Unable to write data");
}
