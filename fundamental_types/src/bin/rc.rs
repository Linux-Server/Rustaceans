use std::rc::Rc;
use std::sync::Condvar;

fn main() {
    // Reference counting smart pointer
    // Its will have multiple owners , when all owner goes out of scope value will be dropped

    let owner_one = Rc::new(10);
    let owner_two = Rc::clone(&owner_one);
    // drop(owner_one);
    println!("The RC in action, {:?}", owner_two);
    println!("the number of owners  {:?}", Rc::strong_count(&owner_two));
    test_list();

}

use crate::List::{Cons,Nil};
use crate::List1::{Cons1, Nil1};
fn test_list(){
    let box_list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("The Box list is {:?}", box_list);

    let rc_list = Rc::new(Cons1(1,Rc::new(Cons1(2, Rc::new(Nil1)))));



}

#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil
}

#[derive(Debug)]
enum List1{
    Cons1(i32, Rc<List1>),
    Nil1
}