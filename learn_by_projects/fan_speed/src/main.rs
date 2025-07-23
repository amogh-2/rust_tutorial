use std::io;
//use std::thread;
//use std::time::Duration;

enum Speed{
    Low,
    Medium,
    High,
}
struct Controller{
    control:Speed,
}

impl Controller{
    fn show(&self){
        match self.control{
            Speed::Low =>{println!("Fan on Low Speed");}
            Speed::Medium=>{println!("Fan on Medium Speed");}
            Speed::High=>{println!("Fan on High Speed");}
        }
    }
    fn increase(&mut self){
        self.control=match self.control{
            Speed::Low => Speed::Medium,
            Speed::Medium => Speed::High,
            Speed::High => {
                println!("Already at Max Speed");
                Speed::High
            }
        };
    }
    fn decrease(&mut self){
        self.control=match self.control{
            Speed::High=> Speed::Medium,
            Speed::Medium=> Speed::Low,
            Speed::Low=>{
                println!("Already at Low Speed");
                Speed::Low
            }
        };
    }
}
fn main(){
    let mut fan=Controller{
        control:Speed::Low,
    };
    
    println!("Fan Speed Controller");
    println!("Use '+' to increase speed, '-' to decrease speed");

    loop{
        fan.show();
        let mut input=String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim(){
            "+"=>fan.increase(),
            "-"=>fan.decrease(),
            "q"=>{
                println!("Existing");
                break;
            }
            _=>{println!("Invalid input");}
        }
    }

}
