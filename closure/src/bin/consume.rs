fn main(){
    let mut x = vec![1,2,3,4,5];

    let total:i32 = x.iter().sum();

    println!("the data {:?}", x);
}