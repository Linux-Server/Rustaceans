use std::ops::DerefMut;
use std::ops::Deref;

fn main(){

    let s = String::from("Sam");
    let z = &s[..];



    let mut x = MyBox::new(String::from("sam"));
    // assert_eq!(10, *(x.deref())); //  *&10
    // assert_eq!(10, *x); //  *&10
     assert_eq!("sam", *(x.deref())); // *&String
     let b = x.deref();
     let c = &x;

    //  let d = *x;

     hello(&mut x); // &MyBox<string> -> coercion-> check Dref trait is impl for MyBox -> true - call ist dref method
     //  -> &String -> call String dref -> rturn  -.>  &str
    //  hello(&(*x)[..]);

}
/*
Here we’re calling the hello function with the argument &x, which is a reference to a MyBox<String> value.
 Because we implemented the Deref trait on MyBox<T> in Listing 15-10,
Rust can turn &MyBox<String> into &String by calling deref. 
The standard library provides an implementation of Deref on String that returns a string slice,
and this is in the API documentation for Deref.
 Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.

*/

//Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type.

fn hello(data: &str){
    println!("The valiue is {:?}", data);
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> MyBox<T>{
    fn new(target:T)-> MyBox<T>{
        MyBox(target)
    }
}