use std::cell::RefCell;

fn main() {
    println!("the ref cell");

    let  x = RefCell::new(10);
    // *x.borrow_mut() = 22;
    // println!("the val is {:?}", x);

    let y  = x.into_inner();

    println!("Get the inner value {:?}", y);


    // assert_eq!(22, *x);
}