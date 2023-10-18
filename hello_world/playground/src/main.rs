mod calc;
mod sub;
use crate::calc::*;

use crate::sub::subtract::sub;

fn main() {
    println!("Hello, world!");
    let x = 10;
    let res = sub(x);
    println!("The res is {:?}", res);
}


