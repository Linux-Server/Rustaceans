
use crate::List::{Cons,Nil};
fn main() {
    let a = 5;
    let b = Box::new(5);
    println!("Hello, world! {:?}", a);

    let list = Cons(10, Box::new(Cons(20, Box::new(Nil))));

    println!("The list is {:?}", list);


}


#[derive(Debug)]
enum List{
    Cons(i32,Box<List>),
    Nil
}
