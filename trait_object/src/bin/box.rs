fn main(){
    println!("hello box");
    let a = Box::new(3);
    println!("The value of a is {:?}", a);
    assert_eq!(3, *a); // deref required , otherwise assertion will fail

    // check Box with vector

    Myvector.0.push(20)

    //  my_vector.push(a);
    //  my_vector.push(3)
}


struct Myvector<T>(Vec<T>);

/*

Box provide inderection and heap allocation

 */


