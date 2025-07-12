use std::io;
fn main(){
    println!("Enter a number: ");
    let mut x=String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Couldn't take input");
    let x:i32 =x.trim().parse().expect("Enter a valid number!");
    match x{
        1 =>{ 
            println!("The number is: {}", x);
        }
        _=>{
            panic!()
        }
    }
}