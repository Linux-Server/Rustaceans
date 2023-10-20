
enum Charactor{
    Superman,
    Ironman,
    Spiderman
}

trait Superpower{
    fn fly(&self)->String;
}

impl Superpower for Charactor{
    fn fly(&self)->String {
        match self{
            Charactor::Ironman => "ironman can fly".to_string(),
            Charactor::Spiderman => "spider can crawl".to_string(),
            Charactor::Superman => "superman can fly".to_string()
        }
    }
}




#[cfg(test)]
mod test{
    use crate::m3_traits::{Charactor, Superpower};
    #[test]
    fn test_traits(){
        dbg!("Test traits");
        let super_hero = Charactor::Spiderman;
        let outs = super_hero.fly();
        dbg!(outs);
    }
}