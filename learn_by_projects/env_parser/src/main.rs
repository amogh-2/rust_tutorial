use std::collections::HashMap;
use std::fs;
use std::env;

fn parse_env(contents: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, '=').collect();

        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        map.insert(key.to_string(), value.to_string());
    }

    map
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run KEY");
        return;
    }

    let key = &args[1];

    let contents = match fs::read_to_string(".env") {
        Ok(c) => c,
        Err(_) => {
            println!("Could not read .env file");
            return;
        }
    };

    let env_map = parse_env(&contents);

    match env_map.get(key) {
        Some(value) => println!("{} = {}", key, value),
        None => println!("Key not found"),
    }
}