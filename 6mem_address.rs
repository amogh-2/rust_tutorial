fn main(){
    let x=String::from("Hello");
    let s=&x;
    println!("The memory address of s is: {:p}", s)     //if we are passing a pointer {:p} gives the memory address of the pointer
}