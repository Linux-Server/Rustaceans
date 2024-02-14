//Ownership is a unique feature in rust lang
// it provides the memory safety without the use of garbage collector
// Ownership is a set of rules that provides memory safety
/*
- Some lang have GC or programmer need to manually allocate/deallocate the memory
- The ownership is sat of rules applied by thee compiler during compile time
- Memory:
    - Both stack and heap are the memory available for a program during runtime
    -   stack: stores value in LIFO (stack of plates)
         - All date stored on stack must have a known and fixed size
         -
    -  Heap :
         -  All data with unknown size is stored in heap
          - Its less organized
          - The memory allocator find the space the heap and returns a pointed(address of location)
          - Bcoz the pointer retuned by the memory allocator have fixed size, we can store that pointer onthe stack
          - Since the stack stores the pointer ,we need to follow th epointer in order to acces the data on the heap

Ownership Rules:
    - Each value in rust have an owner
    - There can be one owner at a time
    - When owner goes out of scope, value will be dropped


*/
fn main(){
    println!("hello Ownership");
    let x = 10;
    copy_value(x);
    println!("The value is {:?}", x);
    let y = String::from("helllo");
    move_value(y);
}

fn copy_value(x:i32){

}

fn move_value(data:String){
    println!("The string is {:?}", data);
}