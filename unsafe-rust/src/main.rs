fn main() {
    let mut my_vec = vec![1,2,3,4,5,6,7,8];

    // let mut my_iter= my_vec.iter();

    assert_eq!(my_vec.next(), Some(1));
    
}

trait Iterator{
    type Item;

    // Required method
    fn next(&mut self) -> Option<Self::Item>;
}


impl Iterator for Vec<i32>{
    type Item=i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some()
        
    }
}
