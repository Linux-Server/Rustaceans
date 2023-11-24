use std::time::{Instant};
fn main() {
    let mut count = 1;
    tester(&mut count);

}
fn tester(count: &mut i32){
    println!("the benchmark of rust {:?}", count);
    let mut my_vec = (1..100).rev().collect::<Vec<i32>>();
    my_vec.sort();
    *count += 1;
}