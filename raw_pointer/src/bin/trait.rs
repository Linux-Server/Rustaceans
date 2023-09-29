fn main(){
    let mut struct_one = Counter{name:"sachin".to_string()};
    // let struct_two = Counter{name:};
    


}

struct Counter{
    name:String
}


// generic iteratot
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}



impl Iterator for Counter{
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
       Some(self.name.clone())
    }
}



