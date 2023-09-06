fn main(){
    let x = 10;
    let res = call_back(&x);
    println!("The res is {:?}", res);
 

}

fn call_back(input: &impl Summary){
   println!("active...")
}

trait Summary{
    fn summarize(&self)-> i32{
        20
    }
}

impl Summary for i32{}
