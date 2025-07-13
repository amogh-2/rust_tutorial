use std::io;
use rand::Rng;
fn main(){
    println!("Enter a number: ");
    let mut x=String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Couldn't take input");
    let x:i32 =x.trim().parse().expect("Enter a valid number!");
    let guess = rand::thread_rng().gen_range(1..=10);
    if x==guess{
        println!("YOU WIN!");
    }
    else{
        println!("Wrong! Now you Die. It was {}",guess);
    }
}
