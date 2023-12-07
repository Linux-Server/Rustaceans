#![allow(dead_code,unused_variables)]

#[derive(Debug)]
pub struct Person{
    name:String
}

impl Person{
    pub fn check(&self, data:String)->String{
        println!("Its a person impl");
        data

    }

    pub fn new(name:String)-> Person{
        Person{
            name
        }
    }
}