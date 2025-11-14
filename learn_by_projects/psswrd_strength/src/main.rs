use std::io;

struct Password{
    value: String,
}

impl Password{
    fn new(value: String) -> Self{
        Password{value}
    }

    fn length_score(&self)-> i32{
        let len = self.value.len();
        if len>=12{
            2
        }
        else if len>=8{
            1
        }
        else {
            -1
        }
    }
    fn has_upperlowercase(&self)-> i32{
        if self.value.chars().any(|c| c.is_uppercase()) && self.value.chars().any(|c| c.is_lowercase()){
            1
        } else{0}
    }
    fn has_digit(&self)->i32{
        if self.value.chars().any(|c| c.is_ascii_digit()){
            1
        } else{0}
    }
    fn has_symbols(&self)->i32{
        if self.value.chars().any(|c| !c.is_alphanumeric()){
            1
        } else{0}
    }
    fn total_score(&self)->i32{
        self.length_score()
            + self.has_digit()
            + self.has_symbols()
            + self.has_upperlowercase()
    }
    fn compare_score(score:i32)-> &'static str{
        match score{
            4..=7 => "Strong",
            2..=3 =>"Medium",
            _ => "Weak",
        }
    }
}

fn main() {
    loop{
        println!("Enter your password: ");
        let mut input= String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input= Password::new(input.trim().to_string());
        let score=input.total_score();
        let rating = Password::compare_score(score);
        println!("Password rating: {}", rating);
        println!("score: {}",score);
        println!("\nPress q to quit or any other key to continue");
        let mut quit = String::new();
        io::stdin().read_line(&mut quit).unwrap();
        let quit= quit.trim().to_string();
        match quit.as_str(){
            "q" => {break;}
            _ =>  {continue;}
        }

    }
}
