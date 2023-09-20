fn main(){
    let person = Person{name:"sachin".to_string()};
    // // bighand
    let Person {name: sam} = person;

    println!("the destruct is {:?}", sam);

    //sugar syntax
    let person1 = Person1{name:"sachin".to_string()};

    let Person1 {name} = person1;

}
#[derive(Debug)]
struct Person{
    name:String
}

#[derive(Debug)]
struct Person1{
    name:String
}