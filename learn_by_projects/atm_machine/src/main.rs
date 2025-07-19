use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let mut operation=0;
    let mut money:f64 = 0.0;
    loop{
        if operation!=0{
            thread::sleep(Duration::from_secs(2));
        }
        operation+=1;
        println!("Enter  (1)To View Balance  (2)To Deposit Money  (3)To Withdraw Money  (4)To Exit");
        let mut press=String::new();
        io::stdin().read_line(&mut press).expect("Error taking input");
        let press = press.trim();
        match press{
            "1" =>{     
                println!("Your Current balance is Rs.{}",money);
            }
            "2" =>{
                println!("How much money to do u wish to deposit?");
                let mut add_money=String::new();
                io::stdin().read_line(&mut add_money).expect("Error taking input");
                let add_money=add_money.trim().parse::<f64>().expect("Enter a valid number");
                if add_money<=0.0{
                    println!("LOl!");
                    continue;
                }
                money+=add_money;
                println!("Money added to your current balance. Your new balance is Rs.{}", money);
            }
            "3" =>{
                println!("How much money do u wish to withdraw?");
                let mut withdraw_money=String::new();
                io::stdin().read_line(&mut withdraw_money).expect("Error taking input");
                let withdraw_money=withdraw_money.trim().parse::<f64>().expect("Enter a valid number");
                if withdraw_money>money{
                    println!("Insufficient Balance.");
                    println!("Your Balance is Rs.{}", money);
                }
                else{
                    money-=withdraw_money;
                    println!("Money withdraw successful. Your current Balance is Rs.{}",money);
                }
            }
            "4" =>{
                println!("Exiting....");
                break;
            }
            _   =>{
                println!("Enter a valid option")
            }
        }

    }
}