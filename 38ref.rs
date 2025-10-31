fn main(){
    let s=String::from("Hello");
    let ref s1=s;
    let s2 = &s;
    assert_eq!(s1,s2);  // ref s1 and &s here both do the same thing
    println!("{}",s)
}