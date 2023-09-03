use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;


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
    if let Err(e) = run(config){
        println!("Apllication error, {:?}",e);
        process::exit(1);
    }



}

