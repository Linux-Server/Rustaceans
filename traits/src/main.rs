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


}

trait Summary{
    fn summarize(&self)-> String;
}

struct Person{
    name:String,
    age: u32
}

impl Summary for Person{
    fn summarize(&self)-> String {
        format!("My name is {} and age is {}", self.name,self.age)
    }
}