fn main() {
    let x = Person {
        name: "sam".to_string(),
    };
    let y = Summary::<i32>::check(&x); //x.check();
    let z = Summary::<String>::check(&x); //x.check();

    let a = Test{
        data:20
    };
    let b =  Test::new();

}


struct Test<T>{
    data: i32
}

impl<T> Test<T>{
    fn new(){
        println!("Its a test");
    }
}
struct Person {
    name: String,
}
impl<T> Summary<T> for Person {
    fn check(&self) {
        println!("This is Summay tarit impl for person");
    }
}

impl<T> SummaryOne<T> for Person {
    fn check(&self) {
        println!("This is Summary One trait impl for person");
    }
}

trait Summary<T> {
    fn check(&self);
}

trait SummaryOne<T> {
    fn check(&self);
}
