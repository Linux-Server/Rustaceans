use std::env;
use std::fs;


fn main() {

    let args: Vec<String> = env::args().collect();
    println!("Args : {:?}", args);  // args[0] is the anme of the program
    let config= Config::new(&args);
   
    //cargo run -- sam ram
    println!("Query passed : {:?}", config.query);  
    println!("File name : {:?}", config.file_path);

    //open the file and read string
    // this should be error handled properly
    let contents = fs::read_to_string("poem.txt").expect("Unable to read file");

    println!("The contents : {:?}", contents);


}




struct Config{
    query:String,
    file_path: String
}

impl Config{
    fn new(args: &[String])->Config{
        Config { query: args[1].clone(), file_path: args[2].clone() }
    }
}
