fn main() {
    println!("fully qaulified syntax");
    let human = Human::new();
    let c = Human::run(&human);
    // both are same , but 2nd one preferred
    println!("The human {:?}", Human::run(&human));
    println!("The human {:?}", human.run());

    let hum_wizard = Wizard::run(&human);


}
#[derive(Debug)]
struct Human{
    id:i32
}

impl Human{
    fn run(&self){
        println!("human can run");
    }

    fn new()->Human{
        Human{
            id:1
        }
    }
}

impl Pilot for Human{
    fn run(&self) {
        println!("Pilot can be human");
    }
}

impl Wizard for Human{
    fn run(&self) {
        println!("Wizard can be human");
    }
}

trait Pilot{
    fn run(&self);
}
trait Wizard{
    fn run(&self);
}