fn main(){
    // Rust is a statically typed language
    // That means, its must know type of  all variables at compile time
    println!("Welcome to data types");
    // Most of the time rust will infer the types
    // sometime, the programmer need to annote the type
    let num:u32 = "100s".parse().expect("Its unable to parse");


}