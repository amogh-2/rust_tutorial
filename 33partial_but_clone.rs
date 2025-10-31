fn main(){
    let t=(String::from("Hello"),String::from("World!"));
    let (s1,s2)= t.clone();
    println!("{:?} {:?} {:?}",s1,s2,t)
}