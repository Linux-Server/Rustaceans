use std::fmt;
use std::fmt::{Formatter, write};

fn main() {
    let x = Wrapper(vec![1,2,3,4]);
    println!("the value is {}", x);

}

struct Wrapper(Vec<i32>);

impl fmt::Display for Wrapper{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!()
    }
}