use std::io;

struct Strin{
    value: String,
    length: usize,
}
impl Strin{
    fn new(input: String)-> Self{
        Strin{value: input.trim().to_string(),length: input.trim().len(),}
    }
    fn display(&self){
        println!("{}'s length is {}",self.value, self.length);
    }
    
}

fn main(){
    println!("Enter the string: ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let y= Strin::new(x);
    y.display();

}