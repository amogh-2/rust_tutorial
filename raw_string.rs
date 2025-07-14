fn main(){
    let x = r"Hello \n world //!\\";
    let y= r#" The computer says "Hello world!" "#;     // r with # let use double inverted commas
    println!("{}",x);
    println!("{}",y);
}