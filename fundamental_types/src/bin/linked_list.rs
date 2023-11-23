use crate::List::{Cons,Nil};
fn main() {
    let  a = 10;

    let x = Cons(10,Box::new(Cons(20,Box::new(Nil))));

    eprintln!("the data is : {:?}", x);

}

#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil
}