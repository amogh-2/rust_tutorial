/*
A marco polo game in Rust. 
The player will be prompted to enter "marco" or "polo". 
If the player enters "marco", the program will respond with "polo". 
If the player enters "polo", the program will respond with "marco". 
The game will continue until the player types "exit".
*/

pub fn marco(input: &str) -> String {
    match input.to_lowercase().as_str() {
        "marco" => "polo".to_string(),
        _ => "Dumbass It's a marco-polo game. You had to say marco!".to_string(),

    }
}