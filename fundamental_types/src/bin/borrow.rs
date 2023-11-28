use std::rc::Rc;
fn main() {
    println!("Smart pointer borrow rules check");
    let box_pointer = Box::new(10);
    // let y = &x;
    // let z = &mut x;
    // println!("The core is {:?}", z);
    let immut_borrow = & *box_pointer;

    println!("The immut borrow {:?}", immut_borrow);

    assert_eq!(10, *immut_borrow);


}