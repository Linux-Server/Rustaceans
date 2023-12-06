use std::time::Instant;

fn main() {
    let person_one: Person<String> = Person::new("sam".to_string());
    println!("The data is {:?}", person_one.tester("sam"));
}
#[derive(Debug)]
struct Person<T> {
    name: T,
}

impl<T> Person<T> {
    fn new(name: T) -> Person<T> {
        Person { name }
    }
}



impl <T, U> Summary<U> for Person<T> {
    type Item = U;
    fn tester(&self, addon: Self::Item) -> Self::Item {
        addon
    }

}

trait Summary<U> {
    type Item;
    fn tester(&self, addon: U) -> Self::Item;
}
