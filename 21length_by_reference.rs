fn main(){
    let mut _s1=String::from("Hello");
    let len = calculate_len(&_s1);
    println!("The length of string is: {}", len);
    
}

fn calculate_len(word:&str)->usize{
    word.len()

}   