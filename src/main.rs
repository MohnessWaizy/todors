use std::fs;
use std::io;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    priority: usize,
    position: usize,
}
// Create a dummy task file for testing purposes
fn create_dummy_task_file(file_path: &str) {
    let dummy_tasks = vec![
        Task {
            name: "Buy groceries".to_string(),
            priority: 3,
            position: 1,
        },
        Task {
            name: "Complete Rust project".to_string(),
            priority: 5,
            position: 2,
        },
        Task {
            name: "Call mom".to_string(),
            priority: 4,
            position: 3,
        },
    ];
    save_tasks(file_path, &dummy_tasks);
}
fn main() {
//    create_dummy_task_file("tasks.json");
    let file_path = "tasks.json";
    let mut task_list = load_tasks(file_path);

    loop {
        let task_name = get_task_name();
        if task_name.eq_ignore_ascii_case("exit") {
            break;
        }

        let priority = get_task_priority();
        let task = create_task(task_name, priority, task_list.len() + 1);
        task_list.push(task);
    }

    display_task_list(&task_list);
    save_tasks(file_path, &task_list);
}

fn load_tasks(file_path: &str) -> Vec<Task> {
    match fs::read_to_string(file_path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(file_path: &str, task_list: &[Task]) {
    let json = serde_json::to_string_pretty(task_list).expect("Failed to serialize tasks");
    fs::write(file_path, json).expect("Failed to write tasks to file");
}

fn get_task_name() -> String {
    println!("Enter task name (or type 'exit' to quit):");
    let mut task_name = String::new();
    io::stdin().read_line(&mut task_name).expect("Failed to read task name");
    task_name.trim().to_string()
}

fn get_task_priority() -> usize {
    loop {
        println!("Enter task priority (1-5):");
        let mut priority_input = String::new();
        io::stdin().read_line(&mut priority_input).expect("Failed to read priority");
        match priority_input.trim().parse() {
            Ok(num) if num >= 1 && num <= 5 => return num,
            _ => println!("Invalid priority. Please enter a number between 1 and 5."),
        }
    }
}

fn create_task(name: String, priority: usize, position: usize) -> Task {
    Task { name, priority, position }
}

fn display_task_list(task_list: &[Task]) {
    println!("\nTask List:");
    for task in task_list {
        println!("{}: {} Priority: {}", task.position, task.name, task.priority);
    }
}
