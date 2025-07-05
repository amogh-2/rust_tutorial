fn main(){
    let mut x = 2; //use mut to make the variable mutable
    x += 3;
    assert_eq!(5,x);    // checks if they are equal
    println!("success!");
}