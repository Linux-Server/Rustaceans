
/// #Euclids Algorithm
/// -There are bmany ways to formalise euclids algo
/// ```
/// let x =10;
/// ```
/// * Ok its a test
/// * Never again
/// * For sure
fn main(){
    //
    //! will check

   let x:u128 =400000000000000000000000000000000000;
   let y: u128 = 10000000000000000000000000000;
   let res = gcd(x,y);
   println!("The greatest common divisor {:?}", res);

}

fn gcd(mut x:u128,mut y:u128)->u128{
   //greatest common divisor
   //a greatet number which can divide both without causing reminder
   while y !=0{
    if y<x{
      let t=x;
      x = y; 
      y =t;
   }
    y = y%x;
    println!("Then {:?} {:?}", y, x);
} 

   return x;


}