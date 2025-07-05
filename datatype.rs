fn main(){
    let x:f32 = 5_i16 as f32;
    println!("x is: {}", x);
    let a=251_u16 + 8;
    let b=i16::checked_add(251,8).unwrap();
    println!("{},{}",a,b);
}