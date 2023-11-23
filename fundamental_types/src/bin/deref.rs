use std::ops::Deref;

fn main(){
    let x = &5;
    let y = Box::new(x);
    println!("its a deref test, {:?}", y);

     assert_eq!(*x,**y);

}

struct MyBox<T>(T, i32);

impl<T> MyBox<T>{
    fn new(val:T)->MyBox<T>{
        MyBox(val, 12)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/*
- Deref trait allow smart pointer to be treated like regular referneces
-
 */

