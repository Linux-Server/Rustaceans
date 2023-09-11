fn main(){
    /*
    -Box smaet pointer allow u to store data on heap rather than stack
    - type isn written as Box<T>
    - what remain on the stack is a pointer to heap
    - Its deosnt have performance overhead other than storing data on heap
    - Capabilities:
        - when u have a type whose size cant be determined on compile time
        - when you have a large amount of data and you want to transfer the ownership
        - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
    
     */

    let a = Box::new(10);
    println!("The value of a is {:?}", a);
}