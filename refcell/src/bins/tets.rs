use std::{rc::Rc,cell::RefCell};

fn main(){

    let mut x = 10;
    let y = &mut x;
    *y = 55;
    


     let x1 = RefCell::new(10);
     let mut z = x1.borrow_mut();
    
    

    
  println!("The test {:?}", z);
}


enum List{
    Cons(RefCell<i32>,Rc<List>),
    Nil
}