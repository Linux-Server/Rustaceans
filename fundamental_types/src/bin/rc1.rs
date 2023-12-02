use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("The rust rc + refcell");
    let ref_owner_one = RefCell::new(10);
    let ref_count = Rc::new(ref_owner_one);
    *ref_count.borrow_mut() = 11;



    println!("The value is {:?}", ref_count);

    println!("The value is {:?}", Rc::strong_count(&ref_count));



}