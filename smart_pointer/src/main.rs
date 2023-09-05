/*
Pointer = Its a varibale that contain address in memory:  ex: reference (just borrow the data) - indicated by & symbol
Smart Pointer = Its a pointer which also have some additional metadata

Pointer vs Smart pointer  => refrence borrow the data while smart pointer owns the data
example of smart pointer -> String and Vec -> Bothyn owns the data and allow u to manipulate it
they also have metadata and other capabilities as well


Box - smat pointer used to store data on heap



*/


fn main() {
    //lets store an i32 value on heap
    let x = Box::new(19);
    println!("Hello, Smart Pointer! {:?}", x);
}
