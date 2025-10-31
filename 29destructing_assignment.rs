fn main(){
    let (x,y); 
    (x,..) = (3,4);
    [..,y] = [5,6];
    assert_eq!([x,y],[3,6]);
    println!("Success");
}