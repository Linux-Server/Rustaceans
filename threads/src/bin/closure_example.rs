fn main() {
    println!("The closure example");

    let my_closure = |num| num + 100;

    println!("{:?}", my_closure(10));
}
