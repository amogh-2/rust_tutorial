use std::fs;


//Reads the content of the file. Err(e) drops, here 'e' stores the error message.
fn main(){
    let content=fs::read_to_string("ownershp.txt");
    
    match content{
        Ok(content) => println!("{}",content),
        Err(e)  =>  println!("Error reading file: {}",e),
    }
}