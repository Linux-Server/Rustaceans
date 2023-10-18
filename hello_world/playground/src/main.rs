mod add;

use crate::add::add;

fn main() {
    println!("Hello, world!");
    let x = 10;
    let res = add(x);
    println!("The res is {:?}", res);
}


