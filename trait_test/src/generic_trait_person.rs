use std::fmt::Debug;

#[derive(Debug)]
pub struct Person<T> {
    name: T
}

impl <T> Person <T>{
pub fn new(name:T)->Person<T>{
        Person{
            name
        }
    }

}

impl<T> Summary for Person<T> where T:Debug{
    fn details(&self) -> String{
        format!("Genreic trait details")
    }

    fn tester(data: String) -> String {
        format!("{:?}", data)
    }
}

pub trait Summary{
    fn details(&self)->String;
    fn tester(data:String)-> String;
}