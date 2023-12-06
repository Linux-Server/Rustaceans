use std::collections::VecDeque;

fn main() {
    let mut x = vec![1,2];
    x.push(3);
    x.pop();
    println!(":The vec is : {:?}", x);

    let mut y  = VecDeque::from([1,2]);
    y.push_back(3);
    // y.pop_front();
    println!(":The vec is : {:?}", y);


}