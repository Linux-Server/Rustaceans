#[derive(Debug)]
pub struct Person<T>{
    name:T
}

impl<T> Person<T>{
    pub fn new(name:T)->Person<T>{
        Person{
            name
        }

    }

   pub fn check<U>(&self, data:U)->U{
        data
    }
}