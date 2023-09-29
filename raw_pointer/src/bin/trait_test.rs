fn main(){
    let person_one = Person{name:"sachin".to_string(), age: 28};
    let res = person_one.profile();
    println!("The res is {:?}", res);


}

struct Person{
    name:String,
    age:i32
}

struct People{
    count: u32
}

trait Summary{
    type Item;
    fn profile(&self)->Self::Item;
}

impl Summary for Person{
    type Item= String;
    fn profile(&self)->Self::Item{
        format!("hello {:?} and your age is {:?}", self.name, self.age)
    }
}

impl Summary for People{
    type Item = u32;
    fn profile(&self)->Self::Item{
        self.count
    }
}