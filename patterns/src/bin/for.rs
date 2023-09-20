fn main(){

    let my_vector = vec![1,2,3,4,5,6];

    for (index,val) in my_vector.iter().enumerate(){
        println!("The value is {:?}", val);
    }

    println!("{:?}", my_vector);
}