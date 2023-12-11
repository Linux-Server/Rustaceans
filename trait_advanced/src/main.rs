use std::iter::Sum;

fn main() {
    println!("Trait Advanced!");
    let a = Person{id:1};
    // let b: String = a.kill();
     let c = a.killer();
     println!("the b is {:?}", c);

}


struct Person{
    id:i32
}

trait Summary<T>{
    fn kill(&self)-> T;
}

trait SummaryOne{
    type Item;
    fn killer(&self)->Self::Item;
}

impl SummaryOne for Person{
    type Item = i32;
    fn killer(&self) -> Self::Item {
        99
    }
}



impl Summary<i32> for Person{
    fn kill(&self) -> i32 {
        10
    }
}

impl Summary<String> for Person{
    fn kill(&self) -> String {
        "sa".to_string()
    }
}