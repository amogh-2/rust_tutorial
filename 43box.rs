fn main(){
    let mut x: Box<i32>= Box::new(5); //allocates 5 to heap memory
    *x=4;   //since x is holding the memory location in the heap i.e is of type Box cannot directly initalize a value to it so we use * to define address pointed by x 
    assert_eq!(*x,4);
    println!("Success!");

}