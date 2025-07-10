use std::io;
fn main(){
    println!("Enter a number: ");
    let mut x= String::new();
    
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let y=x.trim().parse::<i32>().expect("Couldn't convert");

    println!("The next number is {}",y+1);
}
