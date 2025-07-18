use std::env;
fn main(){
    let x: Vec<String> = env::args().collect();
    
    if x.len()!=4{
        eprintln!("Enter as <num1> <op> <num2> (Eg: 5 * 2) ");
        return
    }

    let num1:f32= x[1].parse().expect("Enter a valid number");
    let op=x[2].trim();
    let num2:f32= x[3].parse().expect("Enter a valid number");

    let result=match op{
        "+"=>   num1+num2,
        "-"=>   num1-num2,
        "*"=>   num1*num2,
        "/"=>   if num2!=0.0{
                    num1/num2
                }
                else{
                    eprintln!("Mathematical error");
                    return;
                }
        _=>     {
                    eprintln!("Invalid operation");
                    return;
                }   

    };
    println!("Your answer is: {:?} ",result);
}
