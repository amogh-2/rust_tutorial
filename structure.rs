struct Student{
    name:String,
    _email:String,
    _roll_no:String,
    attendance:bool,
}

fn main(){
    let student_5: Student = Student{
        _email:String::from("amoghbhattarai3205@gmail.comp"),
        name:String::from("Amogh"),
        _roll_no:String::from("PUR078BCT005"),
        attendance:true,
    };
    if student_5.attendance==true{
        let s="present".to_string();
        println!("{:?} is {}",student_5.name,s);    
    }
    else{
        println!("{:?} is absent",student_5.name);
    }
}
