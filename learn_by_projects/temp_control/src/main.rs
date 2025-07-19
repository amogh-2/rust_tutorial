use std::io;
use std::thread;
use std::time::Duration;
//use std::env;

fn main() {
    //let whole: Vec<String> = env::args().collect();

    // if whole.len()!=4{
    //     eprintln!("Enter as <Temperature> <Current scale> <Scale to convert to>");
    //     return;
    //}
    // let temp:f32=whole[1].parse().expect("Invalid number");
    // let init_scale=whole[2].trim();
    // let final_scale=whole[3].trim();
    println!("Enter the Temperature value: ");
    let mut temp= String::new();
    io::stdin().read_line(&mut temp).expect("Error taking input");
    let temp = temp.trim().parse::<f32>().expect("Invalid number");
    println!("What scale is the current temperature in 'C'/'F'/'K' ?");
    let mut init_scale= String::new();
    io::stdin().read_line(&mut init_scale).expect("Error taking input");
    let init_scale=init_scale.trim();
    println!("What scale do u wish to convert it to 'C'/'F'/'K'?");
    let mut final_scale=String::new();
    io::stdin().read_line(&mut final_scale).expect("Error taking input");
    let final_scale=final_scale.trim();
    match final_scale{
        "C" => {

                        match init_scale{
                            "C"    =>{

                                            println!("{} -> {} conversion",init_scale,final_scale);
                                            thread::sleep(Duration::from_secs(3));
                                            println!("The Temp in scale C is: {}",temp);
                            }
                            "F"    =>{
                                            println!("{} -> {} conversion",init_scale,final_scale);
                                            thread::sleep(Duration::from_secs(3));
                                            println!("The Temp in scale F is: {}",(temp-32.0)*5.0/9.0);
                            }
                            "K"    =>{
                                            println!("{} -> {} conversion",init_scale,final_scale);
                                            thread::sleep(Duration::from_secs(3));
                                            println!("The Temp in scale K is: {}",temp-273.15);
                            }
                            _     =>{
                                            println!("Invalid input");
                            }  
                        }
        }
        "F"    =>{
                        match init_scale{
                            "C"    =>{
                                            println!("{} -> {} conversion",init_scale,final_scale);
                                            thread::sleep(Duration::from_secs(3));
                                            println!("The Temp in scale C is: {}",(temp*9.0/5.0)+32.0);
                            }
                            "F"    =>{
                                            println!("{} -> {} conversion",init_scale,final_scale);
                                            thread::sleep(Duration::from_secs(3));
                                            println!("The Temp in scale F is: {}",temp);
                            }
                            "K"    =>{
                                            println!("{} -> {} conversion",init_scale,final_scale);
                                            thread::sleep(Duration::from_secs(3));
                                            println!("The Temp in scale K is: {}",(temp-273.15)*9.0/5.0+32.0);
                            }
                            _     =>{
                                            println!("Invalid input");
                            }  
                        }

        }
        "K"    =>{
                        match init_scale{
                            "C"    =>{
                                           println!("{} -> {} conversion",init_scale,final_scale);
                                            thread::sleep(Duration::from_secs(3));
                                            println!("The Temp in scale C is: {}",temp+273.15);
                            }
                            "F"    =>{
                                           println!("{} -> {} conversion",init_scale,final_scale);
                                            thread::sleep(Duration::from_secs(3));
                                            println!("The Temp in scale F is: {}",(temp-32.0)*5.0/9.0+273.15);
                            }
                            "K"    =>{
                                            println!("{} -> {} conversion",init_scale,final_scale);
                                            thread::sleep(Duration::from_secs(3));
                                            println!("The Temp in scale K is: {}",temp);
                            }
                            _     =>{
                                            println!("Invalid input");
                            }  
                        }
        }
        _     =>{
                            println!("Invalid input");
        }  
    }
}
