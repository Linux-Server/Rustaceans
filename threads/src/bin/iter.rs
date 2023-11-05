fn main() {
    // println!("Hello iterators");
    // let my_vector = vec![1,2,3,4,5];
    // let b = my_vector.iter();
    // println!("The iter is {:?}", b);
    let mut a = Test([1, 2, 3, 4, 5, 6]);

    println!("{:?}", a.next());
    println!("{:?}", a.next());

    // for i in a{
    //     println!("Th ehect...");
    // }
}

struct Test([i32; 6]);

// impl Test{
//     pub fn iter(&self) -> Iter<'_, T> {
//         Iter::new(self)
//     }
// }

impl Iterator for Test {
    type Item = [i32; 6];
    fn next(&mut self) -> Option<Self::Item> {
        println!("The next value is {:?}", self.0);
        Some(self.0)
    }
}
