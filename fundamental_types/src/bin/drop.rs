use std::fmt::Debug;

fn main(){
    println!("Drop trait");
    let a  = MyBox(10);
    let b  = MyBox("hello");
     drop(a);

}

struct MyBox<T:Debug>(T);

impl<T> Drop for MyBox<T> where T:Debug {
    fn drop(&mut self) {
        println!("Drop Activated : {:?}", self.0);
    }
}



