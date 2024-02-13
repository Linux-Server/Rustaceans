fn main() {
    println!("Hello, world!");
    // declare_variable();
    shadowing_ownership();
}

fn declare_variable(){
    // By default all variables are immutable
    // to make it mutable add "mut" keyword
    let mut x = 10; //i32
    x = 300;
    println!("{x}");
    let mut name = "Sam"; // string literal or slice
    name = "ram";
    println!("The name is {:?}", name);
    let mut status = true; // bool
    status = false;
    println!("{status}");
    // concat on string type
    let mut names = "Sam".to_string();
    names = String::from("Ray");
    names.push_str("Jacsdv");
    let chars = 'a';  // char

    let mut name_one = String::new();
    name_one = "Rams".to_string();
    println!("{name_one}");

}

fn shadowing_ownership(){
    // copy
    let x = 10;
    let y = x;

    //shadow
    let x = "sachin";
    let x  = x.len();
    println!("{x}");

    //Ownership
    // if the variable type is
    let x = 10;
    let y = x;
    println!("{x} and {y}");

    let x = "sam".to_string();
    let y = x;
    println!("{x}");


}
