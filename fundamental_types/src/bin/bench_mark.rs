#![allow(dead_code)]

fn main() {
    let mut count = 1;
    tester(&mut count);
}
fn tester(count: &mut i32) {
    let _x = 10;
    println!("the benchmark of rust {:?}", count);
    let mut my_vec = (1..100).rev().collect::<Vec<i32>>();
    my_vec.sort();
    *count += 1;
}

struct Point {
    num: i32,
}
