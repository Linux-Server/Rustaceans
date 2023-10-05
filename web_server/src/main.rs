#![allow(unused_variables, unused_imports)]
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
// --snip--


fn main() {
    //create new instance
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("");
    println!("Server Running on port 7878..");

    // println!("{:?}", listener.incoming());

    for stream in listener.incoming(){
        println!("The stream is : {:?}", stream);
        let stream = stream.unwrap();
        handle_connection(stream);

    }

}

fn handle_connection(data: TcpStream){
    let mut stream = data;
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    println!("Request: {:#?}", http_request);
}


