use std::io;
use std::thread;
use std::time::Duration;
enum Color{
    Red,
    Yellow,
    Green,
}
struct TrafficLight{
    color:Color,
}

impl TrafficLight{
    fn show(&self){
        match self.color{
            Color::Red=>println!("Stop"),
            Color::Yellow=>println!("Wait"),
            Color::Green=>println!("Go"),
        }
    }
    fn next(&mut self){
        self.color=match self.color{
            Color::Red=>Color::Green,
            Color::Green=>Color::Yellow,
            Color::Yellow=>Color::Red,
        }
    }
    fn parse_color(input:&str)-> Option<Color>{
        match input.trim(){
            "Red"=>Some(Color::Red),
            "Green"=>Some(Color::Green),
            "Yellow"=>Some(Color::Yellow),
            _=>None,
        }

    }
}

fn main() {
        println!("What traffic light is it in currently?");
        let mut temp=String::new();
        io::stdin().read_line(&mut temp).unwrap();
        let initial_color=TrafficLight::parse_color(&temp);
        if let Some(color)=initial_color{
            let mut light=TrafficLight{ color };
            
            light.show();
            thread::sleep(Duration::from_secs(10));
            light.next();
            thread::sleep(Duration::from_secs(10));
            light.show();
            thread::sleep(Duration::from_secs(10));
            light.next();
            thread::sleep(Duration::from_secs(10));
            light.show();
        }
        else{
            println!("Invalid Color input.");
        }
    }
