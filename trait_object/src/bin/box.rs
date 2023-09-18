fn main(){
    println!("hello box");
    let a = Box::new(3);
    println!("The value of a is {:?}", a);
    assert_eq!(3, *a); // deref required , otherwise assertion will fail
}

/*

Box provide inderection and heap allocation

 */


