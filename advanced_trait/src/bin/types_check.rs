fn main(){
    type Kilometer = i32;
    let x: Kilometer = 55;
    let y =44;

    println!("The sum id {:?}", x+y);
}


#[macro_export]

macro_rules! sachin {
    ($($x:expr), *) => {
        
    };
}