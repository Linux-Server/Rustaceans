#![allow(dead_code)]
#![allow(unused_variables)]
#[allow(unused_imports)]


use List::{Cons,Nil};
use std::rc::Rc;
fn main(){
    println!("Refrence count");

    let a = Rc::new(Cons(1,Rc::new(Cons(2, Rc::new(Nil)))));
    let b = Cons(10, Rc::clone(&a));
    {
    let c = Cons(100, Rc::clone(&a));
    println!("The Rc value in inner scope {:?}", Rc::strong_count(&a));

    }
    println!("The Rc value is {:?}", Rc::strong_count(&a));



}

#[derive(Debug)]
enum List{
    Cons(i32, Rc<List>),
    Nil
}