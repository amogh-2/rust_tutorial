fn main(){
    let x=5;
    {
        let y=2;
        println!("The value of x is {} and y is {}", x,y);
    }
    let y =3;
    println!("The value of x is {} and y is {}",x,y);
}