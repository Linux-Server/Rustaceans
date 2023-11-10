use std::cell::RefCell;

fn main(){
    let x = RefCell::new(String::from("sam"));
    {
    let mut y = x.borrow_mut();
    y.push_str("data");

    }
    let z = x.borrow();

    println!("The res is {:?}", x);
}