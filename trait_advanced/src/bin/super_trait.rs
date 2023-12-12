use std::fmt;
use std::fmt::Formatter;

fn main() {
    println!("the super trait");
    let x = Person{x:10};
    let y = x.check();

}

struct Person{
    x:i32
}

trait Summary : fmt::Display{
    fn check(&self){
        println!("The value was {}", self);
    }
}

impl Summary for Person{}

impl fmt::Display for Person{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.x)
    }
}