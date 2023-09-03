use std::fs;
use std::error::Error;

pub fn run(config:Config)-> Result<(), Box<dyn Error>>{
    //open the file and read string
    // this should be error handled properly
    let contents = fs::read_to_string(config.file_path)?;

    println!("The contents : {:?}", contents);
    Ok(())
}




pub struct Config{
    pub query:String,
    pub file_path: String
}

impl Config{
    pub fn build(args: &[String])->Result<Config,&str>{
        if args.len() < 3 {
            return Err("not enogh args")
        }
        Ok(Config { query: args[1].clone(), file_path: args[2].clone() })
    }
}
