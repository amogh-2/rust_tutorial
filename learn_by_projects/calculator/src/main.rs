use std::io;
fn main() {
    let mut a=String::new();
    let mut b=String::new();
    let mut op=String::new();
    println!("Enter two numbers to perform operation on");
    println!("Enter the first number: ");
    io::stdin()
            .read_line(&mut a)
        .expect("Failed to load input");
    println!("Enter the second number: ");
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to load input");
    println!("Enter '+' for add '-' for sub '*' for mul and '/' for division: ");
    io::stdin()
        .read_line(&mut op)
        .expect("Failed to load input");
    let a=a.trim().parse::<f32>().expect("Couldn't convert");
    let b=b.trim().parse::<f32>().expect("Couldn't convert");
    let op=op.trim();
    match op{
        "+" => {
            println!("Your answer is: {}",a+b)
        }
        "-" => {
            println!("Your answer is: {}",a-b)
        }
        "*" =>{
            println!("Your answer is: {}",a*b)
        }
        "/" =>{
            if b ==0.0{
                println!("Mathematical Error");
            }
            else{
                println!("Your answer is: {}",a/b)
            }   
        }
        _ =>{
            println!("Invalid operation")
        }
    };
}
