fn main() {
    let x = 5;
    let y = Box::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //Every single value has an owner
    // there can be only one owner at a time
    // when variable goes out of scope, value will be dropped

    {
   
    let y = String::from("hello");   // smart poineter, len, capacity (string )

    let b = y; // shallow copy
    // let b = y.clone() // deep copy
    }


}