fn main(){
    #[derive(Debug)]
    struct Person{
        name:String,
        age:Box<u8>,
    }
    let person:Person=Person {
        name:String::from("Alice"),
        age: Box::new(20),
    };

    let Person{name, ref age} =person;
    println!("The name and age call from person: {} {}",name,age);
    // println!("Age call from Person: {:}",person);
}