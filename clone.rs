fn main(){
    let s= String::from("Hello world");
    prints(s.clone());
    println!("{}",s);

}

fn prints(phrase:String){
    println!("{}",phrase);
}