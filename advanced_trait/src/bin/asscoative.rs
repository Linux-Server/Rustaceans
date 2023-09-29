fn main(){

}

struct Counter{
  val:String
}

trait Test{
    type Item;
    fn test(&self)->&Self::Item;
}

// impl Test for Counter {
//     type Item = i32;
//     fn test(&self)->&Self::Item {
//        &self.val
//     }
// }


impl Test for Counter{
    type Item = String;
    fn test(&self)->&Self::Item {
        &self.val
    }
}