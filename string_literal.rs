fn main(){
    let x: &str="hello world";  // cannot be mutated
    let s1=&x;
    println!("{}",s1);
} 