use std::io;
fn factorial(n:u64)-> u64{
    if n==0 || n==1{    
        1
    }
    else{
        n*factorial(n-1)
    }
}

fn main(){
    println!("Please enter a number: ");
    let mut num = String::new(); 
    io::stdin().read_line(&mut num).unwrap();
    let num = num.trim().parse::<u64>().expect("Parsing went wrong");
    let fact=factorial(num);
    println!("The number is {}",fact);
}
