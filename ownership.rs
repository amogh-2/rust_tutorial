fn main(){

    let s=String::from("Hello"); //string is allocated in heap memory
    take_ownership(s);
    //println!("{}",s) this is not possible as the owner of the string isn't s anymore after the above line but is the function take_ownership()
    let s2=gives_ownership();
    let x=5;    //integer is allocated in stack memory
    makes_copy(x); 
    println!("Printing integer from main: {}", x) //this is possible as the ownership of the integer 5 is just copied to the makes_copy fn and not moved
}

fn take_ownership(a_string:String){
    println!("{} {}", a_string, main::s2);
}

fn makes_copy(an_integer:i32){
    println!("Printing integer from makes_copy fn: {}",an_integer); 
}

fn gives_ownership()-> String{
    let a_string = String:from("World!")
    a_string
}