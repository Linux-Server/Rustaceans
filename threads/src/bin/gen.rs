fn main() {

    let x = Person{name:"sam".to_string()};
    let y= Summary::<i32>::description(&x);
     println!("{:?}", y);
    //<Person as Summary<T>>::description(&x));
    //Summary::<i32>::description(&x);

}
impl Summary<String> for Person{
    fn description(&self) -> String {
        "Hello world".to_string()
    }
}

impl Summary<i32> for Person{
    fn description(&self) -> i32 {
       100
    }
}
struct Person {
    name: String,
}

trait Summary<T> {
    fn description(&self)-> T;


}
