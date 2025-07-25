use std::io;

enum Priority {
    DoItNow,
    DoIt,
    DoItLater,
    DoItIfFree,
}

impl Priority {
    fn check_prior(&self) {
        match self {
            Priority::DoItNow => {
                println!("Do the task right now. Top Priority");
            }
            Priority::DoIt => {
                println!("Do the task");
            }
            Priority::DoItLater => {
                println!("Do the task later");
            }
            Priority::DoItIfFree => {
                println!("Do it when free");
            }
        }
    }
}

struct Task {
    name: String,
    task: Priority,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    println!("Enter the task:");
    let mut a_task = String::new();
    io::stdin().read_line(&mut a_task).unwrap();
    let a_task = a_task.trim().to_string();

    println!("Enter priority:");
    println!("0 - Do it now");
    println!("1 - Do it");
    println!("2 - Do it later");
    println!("3 - Do it if free");

    let mut input_priority = String::new();
    io::stdin().read_line(&mut input_priority).unwrap();
    let input_priority = input_priority.trim();

    let priority = match input_priority {
        "0" => Priority::DoItNow,
        "1" => Priority::DoIt,
        "2" => Priority::DoItLater,
        "3" => Priority::DoItIfFree,
        _ => {
            println!("Invalid input. Defaulting to DoItIfFree.");
            Priority::DoItIfFree
        }
    };

    let task = Task {
        name: a_task,
        task: priority,
    };

    // Save task
    tasks.push(task);

    // Show message
    println!("\nTask added:");
    println!("Name: {}", tasks[0].name);
    tasks[0].task.check_prior();
}
