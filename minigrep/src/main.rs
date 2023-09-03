use std::env;
use std::fs;
use std::process;


fn main() {

    let args: Vec<String> = env::args().collect();
    println!("Args : {:?}", args);  // args[0] is the anme of the program
    let config= Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
   
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
    fn build(args: &[String])->Result<Config,&str>{
        if args.len() < 3 {
            return Err("not enogh args")
        }
        Ok(Config { query: args[1].clone(), file_path: args[2].clone() })
    }
}
