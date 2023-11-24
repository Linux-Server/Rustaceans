use std::time::{Instant};
fn main() {
    println!("the benchmark of rust");
    let mut my_vec = (1..100).rev().collect::<Vec<i32>>();
    println!("{:?}",my_vec);
    let start_time = Instant::now();

     my_vec.sort();

    let elspsed_time = start_time.elapsed();

    println!("The running time is {:?}", elspsed_time);

}