#![allow(dead_code)]
#![allow(unused_variables)]

use macros::{Profile,my_attribute};


fn main() {
    let person_one = Person{name:11, age:33};
    person_one.name();
    new();
   
}

#[my_attribute(check)]
fn test(){

}

#[derive(Profile)]
struct Person {
    name: u32,
    age: u32,
}

trait Profile{
    fn name(&self);
}

