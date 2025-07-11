fn main(){
    #[derive(Debug)]
    struct Student{
        name: String,
    }
    let x= Student{name : "Amogh".into()};
    println!("{}",x.name);
}