use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <log_file_path>");
        return;
    }

    let file_path = &args[1];

    
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);

    let mut log_level_count: HashMap<String, usize> = HashMap::new();
    let mut ip_count: HashMap<String, usize> = HashMap::new();
    let mut total_lines = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        total_lines += 1;

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 3 {
            continue;
        }

        let level = parts[2].to_string();
        *log_level_count.entry(level).or_insert(0) += 1;

    
        for part in &parts {
            if part.starts_with("IP=") {
                let ip = part.trim_start_matches("IP=").to_string();
                *ip_count.entry(ip).or_insert(0) += 1;
            }
        }
    }

    
    println!("\nTotal Lines: {}\n", total_lines);

    println!("Log Levels:");
    for (level, count) in &log_level_count {
        println!("{}: {}", level, count);
    }

    println!("\nIP Counts:");
    for (ip, count) in &ip_count {
        println!("{}: {}", ip, count);
    }
}