fn main(){
    // sachin!(10+20, 33+44);
    // anju!(10,30,55,);
    multiples!(1,2);

}


#[macro_export]
macro_rules! sachin {
    ($name:expr, $second:expr) => {
        println!("Hello buddy {:?}", $second);
        
    };
}


#[macro_export]
macro_rules! anju{
    ($($name:expr),* $(,)?)=>{
        $(
            println!("hellio anju : {:?}", $name);
        )*
     
    }
}


#[macro_export]
macro_rules! multiples{
    ($a:expr)=>{
        println!("The one args are {:?}",$a);

    };
    ($a:expr ,$b:expr)=>{
        println!("The two args are {:?} and {:?}", $a, $b);

    }
}
//U BROKE THE RULE, DUMBASS