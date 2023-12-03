use std::cell::{RefCell};
use std::rc::Rc;
use crate::List::{Cons,Nil};
fn main(){

    let mut z = Box::new(10);
    *z = 12;

    let mut x = Rc::new(10);
    // *x = 22;

    let d = RefCell::new(10);

    let a = Cons(1, Rc::new(Cons(2, Rc::new(Nil))));

    let mut h = Test::Red(10);
    let c = &mut h;


}

enum List{
    Cons(i32, Rc<List>),
    Nil
}

enum Test{
    Red(i32),
    Green(String),
    Blue(bool)
}