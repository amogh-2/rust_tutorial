use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let id="Amogh".to_string();
    let password="amogh,123".to_string();
     let mut attempt=3;
    while attempt>0{
        println!("Enter id: ");
        let mut get_id=String::new();
        io::stdin().read_line(&mut get_id).expect("Error reading line");   
        let get_id=get_id.trim();
        println!("Enter password: ");
        let mut get_pass=String::new();
        io::stdin().read_line(&mut get_pass).expect("Error reading line");   
        let get_pass=get_pass.trim();
       
            if id==get_id && password==get_pass{
                println!("Successful login!");
                thread::sleep(Duration::from_secs(2));
                break;
            }
            else{
                attempt-=1;
                if attempt==0{
                    println!("Too many unsuccessful attempts. Try again later!");
                    thread::sleep(Duration::from_secs(2));
                }
                else{
                    println!("Incorrect id or password. {} {} left",attempt, if attempt==1 {"attempt"} else{"attempts"});
                }
            }
    }
}
