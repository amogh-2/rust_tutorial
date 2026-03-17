use std::fs::OpenOptions;
use std::io::Write;


fn main(){
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("zebra.txt")
        .expect("Failed to open file");
    
    writeln!(file,"New log entry").expect("Write Failed");
}