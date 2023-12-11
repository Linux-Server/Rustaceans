fn main() {

    let a = Dog::baby();
    println!("The a is {:?}", a);

    let b = <Dog as Animal>::baby();
    println!("The b is {:?}", b);

}

struct Dog;

trait Animal{
    fn baby()->String;
}

impl Dog{
    fn baby()->String{
        format!("Homw baby")
}
}

impl Animal for Dog{
    fn baby() -> String {
        format!("Animal baby")
    }
}

impl Animal for i32{
    fn baby() -> String {
        format!("kiler")
    }
}