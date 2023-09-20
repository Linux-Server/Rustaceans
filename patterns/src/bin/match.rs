fn main(){
    //multiple match
    let x = 10;
    match x {
        10|20 => println!("The value is 10 or 20"),  // or operator
        _ => println!("not the expected value")
    }

    //range matching

    match x{
        1..=5 => println!("Not bwtwenn 1 to 4"),
        9..=12 => println!("Got the expected value"),
        _ => println!("Not found")
        }
}