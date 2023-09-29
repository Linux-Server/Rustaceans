use std::ops::Add;

fn main(){
   let x = Point{x:20,y:30} + Round{ x:80,y:70};
   println!("Th eval {:?}", x);
}
#[derive(Debug)]
struct Point{
    x:i32,
    y:i32
}

struct Round{
    x:i32,
    y:i32
}


impl Add<Round> for Point{
    type Output = Point;
    fn add(self, rhs: Round) -> Point {
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y 
        }
    }
}
