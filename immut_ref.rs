fn main (){
    let mut x = String::from("HELLO");
    {
        let s1= &x ; 
        let s2=&x;  // can initialize more then one immutable references of the same variable in the same scope   
        //let s2= &mut x; couldn't have initialized a mutable references of the same variable  in the same scope
        println!("{} {}",s1,s2);  
    }
    let s2= &mut x;
    println!("{} world!",s2);
}