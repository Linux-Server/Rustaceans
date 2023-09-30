use std::ops::Add;

fn main(){

   let x = Millimeter(20);
   let y = Meter(55);
   let res = x.add(y);

   println!("TRhe result is {:?}", res);
}

#[derive(Debug)]
struct Millimeter(u32);
struct Meter(u32);

impl Add<Meter> for Millimeter{
    type Output = Millimeter;
    fn add(self, other: Meter)-> Millimeter{
        Millimeter(20+44)
    }
}

