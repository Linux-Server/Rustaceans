use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("Args : {:?}", args);  // args[0] is the anme of the program

    let query = &args[1];
    let file_path = &args[2];
//cargo run -- sam ram
    println!("Query passed : {:?}", query);  
    println!("File name : {:?}", file_path);


}
