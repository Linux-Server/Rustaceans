fn main(){
    let person = Person{name:"sachin".to_string()};

    let Person {name: sam} = person;

    println!("the destruct is {:?}", sam);

}
#[derive(Debug)]
struct Person{
    name:String
}