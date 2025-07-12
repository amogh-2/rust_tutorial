fn main(){
    let s1:String=String::from("HELLO ");
    let s2:&str="WORLD";
    let s:String=s1+s2;    // during concatination the first string should be type string and second should be type string slices
    println!("{}",s);