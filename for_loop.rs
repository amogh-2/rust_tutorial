fn main(){
    let mut sum:i32 = 0;
    for i in -3..2{ //from -3 to 1
        sum+=i;
    }
    assert_eq!(sum,-5);
    println!("success!");    //assert_eq!(var1,var2); but here just assert! is used so ==
        for c in 'a'..='f'{
            println!("{}",c);

        }
}