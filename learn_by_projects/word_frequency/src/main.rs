use std::fs;
use std::collections::HashMap;

fn main() {
    let text = fs::read_to_string(r"../../ownership.txt")
        .expect("Cannot read file");

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}