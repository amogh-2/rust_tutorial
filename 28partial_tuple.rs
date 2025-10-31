fn main(){
    let t:(String,String) = (String::from("Hello"), String::from("World!"));
    let s= t.0;
    println!("{} {}",s,t.1);
}