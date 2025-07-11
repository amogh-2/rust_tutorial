use std::io;
fn main(){
    let mut s1=String::new();
    println!("Enter your string");
    io::stdin()
        .read_line(&mut s1)
        .expect("Couldn't read input");
    let no_spaces= s1.trim();
    let len = calculate_len(&no_spaces);
    println!("The length of string is: {}", len);
    
}

fn calculate_len(word:&str)->usize{
    word.chars()        // outs char from string one by one
     .filter(|c| !c.is_whitespace())    // c for character, !c is_whitespace() if c isn't whitespace returns true and is filtered in
     .count()   // counts from filtered

}   