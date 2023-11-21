/*
  Pointer => a variable that contain address in memory
  - example : Reference (&) - its borrow the value they point to
  Smart Pointer => Act like a pointer + addition metadata and capabilities
   - In many cases reference only borrow the data but Smart pointer owns the data
   - SP is usually implements as struct
   -Unlike ordinary structs SP implemenst Deref and drop traits
   - example : Box, String ,Vec, Rc
   - Deref traits allow the instance smart pointer to act like regular reference or SP
   - Drop traits allow you to customise code thats run , when SP goes out of scope

   Types is Sp:
     Box - Allocate value on heap and provide indirection
     Rc - refernce counting (to have multiple owners for a single value
     RefCell - an Sp enforces borrowing rules at runtime rather than compile time


  */

fn main(){
    println!("WELCOME TO SMART POINTERS");
    box_smart_pointer();
}


fn box_smart_pointer(){
    /*
     Box Smart Pointer- Box<T>
     ex : let x = Box::new(10);
     - its stores data on heap rather than stack
     - what remains on the stack is a pointer to the heap
     - Use box on the following scenarios
            - When u have a type whose size is unknown at compile time
            - When u have large amount of data and wants to transfer ownership end ensure data wont be copied
            -  (Trait Object) When u want to own a value and you only care about the type that impl a trait rather than a fixed type



     */
    println!("The box smart pointer");
}