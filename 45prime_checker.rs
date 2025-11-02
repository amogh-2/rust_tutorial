use std::io;

fn checker(x: i32) -> String {
    if x <= 1 {
        return "The number isn't Prime".to_string();
    }


    for i in 2..=((x as f64).sqrt() as i32) {
        if x % i == 0 {
            return "The number isn't Prime".to_string(); 
        }
    }

    "The number is Prime".to_string() 
}

fn main() {
    let mut x = String::new();
    println!("Enter the number to check: ");
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().parse::<i32>().expect("Not a valid number");

    println!("{}", checker(x));
}
