//CLOSURE
/*
Its an anonymous func that can be saved on varibale and pass as a function argumnet


*/

fn main() {
    println!("Hello, Closure!");
// closure returning an i32 value
    let my_closure = || 10;
    let x = my_closure();
    println!("The closure {:?}", x);
}
