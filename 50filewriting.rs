use std::fs;

//Replaces the content with something new. If the file doesn't exist creates a new one too.
fn main(){
    fs::write("zebra.txt","This is another zebra").expect("Failed to write in the file");
}


