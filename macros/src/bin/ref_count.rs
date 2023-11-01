use List::{Cons,Nil};

fn main(){
    println!("Reference count");

    let a = Cons(1,Box::new(Cons(2,Box::new(Nil))));


    println!("the value is {:?}", a);
}



#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil
}