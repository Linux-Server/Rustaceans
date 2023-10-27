use std::ops::{Deref};
fn main(){
    println!("Treating smart poonter like regular reference with deref trait");
    let x = MyBox::new(5);
    println!("the value is {:?}", x);

     assert_eq!(5,*x);
  
   
}



#[derive(Debug)]
struct MyBox<T>(T);

impl<T>Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

impl<T> MyBox<T>{
    fn new(data:T)->MyBox<T>{
        MyBox(data)
    }
}

