fn main(){
    let person_one = Person{name :"sam".to_string()};
    person_one.talk();
    Female::talk(&person_one);

    ////////////////
    Person::talk1();
    <Person as Male>::talk1();
    <Person as Female>::talk(&person_one)

}

struct Person{
    name:String
}

trait Male{
    fn talk(&self){
        println!("Male is talking");
    }

    fn talk1(){
        println!("Male is talking");
    }
}

trait Female{
    fn talk(&self){
        println!("Female is talking");
    }
    fn talk1(){
        println!("Female is talking");
    }
}

impl Person{
    fn talk(&self){
        println!("person is talking...")
    }

    fn talk1(){
        println!("person is talking...")
    }
}

impl Male for Person{}
impl Female for Person{}