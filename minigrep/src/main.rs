use std::env;
use std::fs;


fn main() {

    let args: Vec<String> = env::args().collect();
    println!("Args : {:?}", args);  // args[0] is the anme of the program

    let query = &args[1];
    let file_path = &args[2];
//cargo run -- sam ram
    println!("Query passed : {:?}", query);  
    println!("File name : {:?}", file_path);

    //open the file and read string
    let contents = fs::read_to_string("poem.txt");

    println!("The contents : {:?}", contents);


}
