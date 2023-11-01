use List1::{Cons,Nil};

fn main(){
    println!("Reference count");
    // let a = Cons(1,Box::new(Cons(2,Box::new(Nil))));
    let node1 = List1::Cons(1, &List1::Cons(2, &List1::Nil));

    println!("the value is {:?}", node1);
}



// #[derive(Debug)]
// enum List{
//     Cons(i32, Box<List>),
//     Nil
// }

#[derive(Debug)]
enum List1<'a>{
    Cons(i32, &'a List1<'a>),
    Nil
}

//We use the Rc<T> type when we want to allocate some data on the heap for
//multiple parts of our program to read and
// we can’t determine at compile time which part will finish using the data last