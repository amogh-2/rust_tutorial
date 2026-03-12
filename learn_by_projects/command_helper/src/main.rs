use std::collections::HashMap;

fn main() {
    let mut commands = HashMap::new();

    commands.insert("scan", "scans open ports");
    commands.insert("sniff", "captures packets");
    commands.insert("block", "blocks an IP");
    commands.insert("log", "shows logs");

    let input = "scan";

    match commands.get(input) {
        Some(desc) => println!("{}: {}", input, desc),
        None => println!("Command not found"),
    }
}