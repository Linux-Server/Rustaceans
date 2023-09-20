fn main(){
    //multiple match
    let x = 10;
    match x {
        10|20 => println!("The value is 10 or 20"),  // or operator
        _ => println!("not the expected value")
    }
}