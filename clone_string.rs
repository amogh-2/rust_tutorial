fn main(){
    let s1 =String::from("Hello");
    let s2=s1;                          //from here s1 doesn't own string "Hello" the ownership of the string was not copied but moved from s1 s2
    let s3=String::from("world!");
    let s4=s3.clone();
    println!("s2: {} s3: {} s4: {}",s2,s3,s4);

}