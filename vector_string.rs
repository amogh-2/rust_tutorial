fn main(){
    let mut x:Vec<String>= Vec::new();
    x.push("String_1".to_string());
    x.push("String_2".to_string());
    x.push("String_3".to_string());
    for i in 0..x.len(){
        println!("{}",x[i]);
    }
    
}