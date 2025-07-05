fn main(){
    let mut x=2;
    x=7;
    x+=2;
    println!("x is: {}",x);
    let y = 2;
    println!("y is: {}",y);
    //shadowing
    let y ="Hello world";   //when datatype is changed it is recorded as a different variable
    println!("{}",y);
}