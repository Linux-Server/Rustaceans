use std::fmt::Write;
fn main() {
    let mut buffer = String::from("hello");
    let mut b2 = vec!["a","b","c"];
    write!(buffer, " world");
    write!(&mut b2, "d");
    println!("The value of x {:?}", b2);
}