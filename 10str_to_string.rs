fn main(){
    let x:&str="hello world";
    give_back(x.to_string());
}

fn give_back(phrase:String){
    println!("{}",phrase)
}