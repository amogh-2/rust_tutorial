fn main(){
    let s= String::from("Hello world");
    prints(&s);
    println!("{}",s);
}
fn prints(phrase:&String){
    println!("{}",phrase);
}

