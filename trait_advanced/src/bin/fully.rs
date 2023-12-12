fn main() {

    let a = Dog::baby();
    println!("The a is {:?}", a);

    let b =  <Dog as Animal>::baby();
    println!("The b is {:?}", b);

    let c = Dog.baby1();

    let d = Animal::baby1(&Dog);

}

struct Dog;

trait Animal{
    fn baby()->String;
    fn baby1(&self);
}

impl Dog{
    fn baby()->String{
        format!("Homw baby")
}
    fn baby1(&self){
        println!("Its a self home baby");
    }
}

impl Animal for Dog{
    fn baby() -> String {
        format!("Animal baby")
    }

    fn baby1(&self) {
        println!("Its a animal self");
    }
    
}
