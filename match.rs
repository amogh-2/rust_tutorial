fn main(){
    get_option(3);
}    
fn get_option(tp:u8){
    match tp{
        1   => {
            println!("Execute case 1")
        }
        2   => {
            println!("Execute case 2")
        }
        _   => {
            never_return();
        }
    };
}
fn never_return()->!{
    panic!()
}