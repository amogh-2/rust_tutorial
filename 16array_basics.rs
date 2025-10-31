fn main(){
    let an_array:[i32;4] =[2,5,6,1];
    let an_index=an_array[2];
    let b=&an_array[0..2];  // & for slices
    assert_eq!(an_index,6);
    println!("{:?}",b);
    println!("Success!");
}