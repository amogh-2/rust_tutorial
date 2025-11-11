use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    println!("\n\n\nYou are an underground hacker attempting to infiltrate a classified AI network.\nYou start at Trace Level 0 — if your trace level hits 3, you get caught. \nYour goal is to reach the Core Node by passing through all 3 security layers.\n\nPress Enter to continue or q to quit. ");
    let mut enter_press= String::new();
    io::stdin().read_line(&mut enter_press).expect("Error taking input");
    let enter_press=enter_press.trim();
    if enter_press.is_empty(){
        start_game();
    }
    else{
        println!("See you next time then Agent, Adios...");
    }
}

fn start_game(){
    println!("Initializing secure terminal...\nConnecting to network...\nAccessing mainframe [OK].\nWelcome, Agent 0x75B.");
    loop{
        println!("\nPress: ");
        println!("1. To Scan Network\n2. To Attack Node\n3. To View Status\n4. Exit");
        let mut press= String::new();
        io::stdin().read_line(&mut press).expect("Error taking input");
        let press=press.trim().parse::<i32>().expect("Invalid number");
        match press{
            1 => {
                    println!("Execute situation 1");
                    thread::sleep(Duration::from_secs(2));
            }
            2 => {
                    println!("Execute situation 2");
                    thread::sleep(Duration::from_secs(2));
            }
            3 => { 
                    println!("Execute situation 3");
                    thread::sleep(Duration::from_secs(2));
            }
            4 => {
                    println!("Exiting...");
                    thread::sleep(Duration::from_secs(2));
                    break;
            }
            _ =>{
                println!("Invalid Input");
            }
        }
    }
}