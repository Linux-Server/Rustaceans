use std::iter::Sum;

fn main() {
    println!("Default Generic Type..");

    let x = Person{id:1};
    let y = Person{id:10};
    let b = x.tester(y);
    println!("The b is {:?}", b);
}
trait Summary<Rhs=Self>{
    type Item;
    fn tester(&self, other:Rhs)-> Self::Item;
}

#[derive(Debug)]
struct Person{
    id:i32
}



impl Summary for Person{
    type Item = Person;
    fn tester(&self, other: Person) -> Self::Item {
        Person{
            id:33
        }
    }
}