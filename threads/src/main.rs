#![allow(unused_variables, dead_code)]

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let a = String::from("sachin");
    let x = thread::spawn(move || {
        for i in 1..10 {
            println!("My threads : {:?}", a);
            thread::sleep(Duration::from_millis(1000))
        }
    });
    x.join().unwrap();

    for i in 1..10 {
        println!("The main thread");
        thread::sleep(Duration::from_millis(1000));
    }
}
