use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("poem.txt")?;
    println!("the content : {:?}", f);
    let reader = BufReader::new(f);

    let line = reader.lines();

    let res = line.map(|result| result.unwrap());

     let data = test();

     for i in data{
        println!("the outcome is {:?}", i);

     }

    Ok(())
}

fn test()-> impl Iterator<Item=i32>{
    let a = [-1i32, 0, 1];
     let mut iter = a.into_iter().take_while(|x| x.is_negative());

    let v1 = [1,2,3,4];
    let v3 = v1.into_iter().map(|data| data).take_while(|x| x>&2);
    v3
    
}