#![allow(unused_variables, dead_code, unused_imports)]
use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();
    for args in env::args().skip(1) {
        println!("The args are {:?}", args);
        numbers.push(u64::from_str(&args).expect("Nothing"));
    }

    println!("the vector have {:?}", numbers);
}
