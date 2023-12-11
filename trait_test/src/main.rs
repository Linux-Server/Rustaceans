
use trait_test::Person;

mod generic_person;
mod trait_person;
mod advanced;
mod generic_trait_person;
use generic_person::Person as Gperson;
use trait_person::{Person as Tperson, Summary, SummaryOne};
use generic_trait_person::{Person as GTperson, Summary as GTSummary};
use crate::advanced::one::Sachin;


fn main() {

    let x = Sachin{name:"Killer".to_string()};


}

fn test1(){
    let person = Person::new("sam".to_string());
    println!("The person is {:?}", person);
    println!("");
    println!("The person is {:?}", person.check("ray".to_string()));

}

fn test2(){
    let x = Gperson::new(100);
    println!("The Gpay {:?}", x);

    let y = x.check("Sachin".to_string());
    println!("Its a {:?}", y);
}

fn test3(){
    let x = Tperson::new("Samray".to_string());
    println!("The val is {:?}", x);
    let details = x.details();
    println!("The deta {:?}", details);
    let z = Summary::details(&x);
    println!("The rays {:?}", z);

    let t = <Tperson as Summary>::tester("raymond".to_string());
    println!("The tester is {:?}", t);
}

fn test4(){
    let x = GTperson::new(200);
    println!("the GT instance {:?}", x);
    let y = x.details();
    println!("The y is {:?}", y);
    let z = GTperson::<String>::tester("ram".to_string());
    println!("The associate {:?}", z);
}