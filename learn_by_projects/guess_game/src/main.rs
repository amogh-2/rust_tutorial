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
    match x.cmp(&guess){
        std::cmp::Ordering::Less => println!("Too small, the num is {}", guess),
        std::cmp::Ordering::Greater => println!("Too Big! the num is {}", guess),
        std::cmp::Ordering::Equal => println!("You win!"),
    }
}
