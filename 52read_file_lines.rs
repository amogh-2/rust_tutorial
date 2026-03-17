use std::fs::File;
use std::io::{BufRead,BufReader};
use std::fs;

fn create_file(){
    let a_file = fs::write("zebra.txt","This is a zebra.\nThe zebra lives in the jungle.\nThe zebra is an herbivore.").expect("Failed to write");
}

fn main(){
    create_file();
    let file = File::open("zebra.txt");

    match file{
        Ok(file) =>{
            let reader = BufReader::new(file);

            for line in reader.lines(){
                let line = line.expect("Error reading line");
                println!("{}",line);
            }
        },
        Err(e)  => println!("{}",e),
    }
}