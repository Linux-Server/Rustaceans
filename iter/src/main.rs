fn main() {
    
    // let person_one = Person {name: "sam".to_string()};  // converted Person struct into a String type
    // let name= person_one.to_user();
    // name.sachin();

    let a = vec![1,2,3,4,5];
    let b = a.iter().map(|x| x);
    println!("the data , : {:?}", b);
  

}


struct Person{ // type
    name:String
}

struct User{ //type
    username: String
}

impl Person{
    fn to_user(&self)-> User{
        let x = self.name.clone();
        User{
            username: x
        }
    }
}

trait Iterator2{
    fn sachin(&self)-> String;
}

impl Iterator2 for User{
    fn sachin(&self)->String{
        "sachin".to_string()
    }
}

