#[derive(Debug)]
pub struct Person{
    name:String
}

impl Person{
    pub fn new(name:String)-> Person{
        Person{
            name
        }
    }
}

impl Summary for Person{
      fn details(&self) -> String {
        format!("Sachin")
    }
      fn tester(data: String) -> String {
        format!("{:?}", data)
    }
}

pub trait Summary {
    fn details(&self)-> String;

    fn tester(data: String) -> String;
}