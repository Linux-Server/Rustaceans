fn main(){

}

struct Person{
    age:String
}


trait Identity{
    type Item;
    fn profile(&self)->Self::Item; 
}

impl Identity for Person{
    type Item = String;
    fn profile(&self)->Self::Item{
        self.age.clone()
    }
}

