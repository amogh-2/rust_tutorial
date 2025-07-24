use std::io;

enum Priority{
    DoItNow,
    DoIt,
    DoItLater,
    DoItIfFree,
}

struct Task{
    name:String,
    task:Priority,
}

impl Priority{
    fn check_prior(&self){
        match self{
            Priority::DoItNow => {
                println!("Do the task right now. Top Priority");
            },
            Priority::DoIt=>{
                println!("Do the task");
            },
            Priority::DoItLater=>{
                println!("Do the task later");
            },
            Priority::DoItIfFree=>{
                println!("Do it when free");
            },
        }
    }
}    


fn main() {
    println!("Enter the task:");
    let mut tasks:Vec<String>=Vec::new();
    let mut a_task=String::new();
    io::stdin().read_line(&mut a_task).unwrap();
    tasks.push(a_task.trim().to_string());
    println!("The task is added.");
    println!("Enter '0' for if u need to do the tasks right now.");
    println!("Enter '1' for if u need to do the tasks.");
    println!("Enter '2' for if u need the task to be done later.");
    println!("Enter '3' for if u need the task to be done when free.");
    let mut x=String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x=x.trim();
    match x{
        "0"=>{Priority::DoItNow}
    }
}
