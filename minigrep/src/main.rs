use std::env;
use std::fs;


fn main() {

    let args: Vec<String> = env::args().collect();
    println!("Args : {:?}", args);  // args[0] is the anme of the program
    let config= parse_config(&args);
   
    //cargo run -- sam ram
    println!("Query passed : {:?}", config.query);  
    println!("File name : {:?}", config.file_path);

    //open the file and read string
    // this should be error handled properly
    let contents = fs::read_to_string("poem.txt").expect("Unable to read file");

    println!("The contents : {:?}", contents);


}


fn parse_config(args: &[String])->Config{
    let query = args[1].clone();
    let file_path = args[2].clome();
    Config{query,file_path}

}

struct Config{
    query:String,
    file_path: String
}
