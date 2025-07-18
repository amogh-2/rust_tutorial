use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    let mut first_run=0;
    loop{
    if first_run!=0{
        thread::sleep(Duration::from_secs(3));
    }
        println!("\n");
        println!("TODD list Manager");
        println!("1. ADD a task");
        println!("2. View tasks");
        println!("3. Mark task as completed");
        println!("4. Delete a task");
        println!("5. Quit");
        println!("\nEnter your choice:  ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading choices");
    let choice=choice.trim();
    match choice{
        "1"=> { 
                let mut a_task=String::new();
                println!("\nEnter the task: ");
                io::stdin().read_line(&mut a_task).expect("Error taking input");
                tasks.push(a_task.trim().to_string());
                println!("\nTask added.");
            },
        
        "2"=>  {
                if tasks.iter().all(|t| t.trim().is_empty()){
                    println!("\nThe task list is empty!");
                }
                else{
                    for (i,a_task) in tasks.iter().enumerate(){
                        println!("\nTask number {} is {}",i+1,a_task);
                    }
                }
            },   
        
        "3"=>   {
                    println!("\nEnter the task number you've completed: ");
                    let mut task_no = String::new();
                    io::stdin().read_line(&mut task_no).expect("Error taking input");
                    let task_no=task_no.trim().parse::<usize>().expect("Invalid number");
                    if task_no > tasks.len() || task_no ==0{
                        println!("\nTask with that task number doesn't exist");
                    }
                    else{
                        tasks[task_no-1]+=" (Task Completed)";
                        println!("\nTask no {} is marked complete.",task_no);
                    }
            },
        "4"=>   {
                    let mut task_no = String::new();
                    println!("\nEnter the task number to be deleted: ");
                    io::stdin().read_line(&mut task_no).expect("Error taking input");
                    let task_no=task_no.trim().parse::<usize>().expect("Invalid number");
                    if task_no > tasks.len() || task_no ==0{
                        println!("\nTask with that task number doesn't exist.");
                    }
                    else{
                        tasks.remove(task_no-1);
                        println!("\nTask no {} is removed", task_no);
                    }
            },
        "5"=>   {
                    println!("\nExiting..");
                    break;
            },
        _=>     {
                    println!("Invalid input")
            }
    } first_run+=1;
    }
    
}

