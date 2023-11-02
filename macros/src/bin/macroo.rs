use crate::identitys;
fn main(){
    // sachin!(10+20, 33+44);
    // anju!(10,30,55,);
    // multiples!(a:1,b:2);
    let num = 30;
    identitys!(num);


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
    (a:$a:expr)=>{
        println!("The one args are {:?}",$a);

    };
    (a:$a:expr ,b:$b:expr)=>{
        println!("The two args are {:?} and {:?}", $a, $b);

    }
}


#[macro_export]
macro_rules! identitys{
    ($num:ident)=>{
        println!("The identity is {:?}", $num);
    }
}
//U BROKE THE RULE, DUMBASS