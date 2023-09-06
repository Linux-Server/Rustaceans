/*
1. trait is behaviour for particluyar type can have
2.we can use trait bound- generic type can be any type that has certain behaviour.
3.Trait is similar to interface in other prog languages


*/


fn main(){
  
  let person_one = Person{
    name:"Alice".to_string(), age:22
  };

  let data = person_one.summarize();
  println!("{:?}", data);
//   person_one.fall_back();


}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
struct Person{
    name:String,
    age: u32
}

impl Summary for Person {
    fn summarize_author(&self) -> String {
        format!("@{}", self.name)
    }
}