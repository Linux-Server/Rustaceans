use std::mem::take;

fn main() {
    let x = vec![1,2,3,4,5];
    // all method
    let mut x_iter = x.iter();
    let z = x_iter.all(|&val| val>0);
    assert!(z);

    let z = x_iter.by_ref().all(|&val| val>0);
    assert!(z);
    println!("by actiuon");

    let mut y =  vec![1,2,3,4,5].into_iter();
    let z = y.by_ref().take(1).collect::<Vec<i32>>();
    println!("The expcetd {:?}", z);
    println!("The expcetd {:?}", y);

    let x = vec![1,2,3,4,5];
    // let x = x.by_ref();

    for i  in x.into_iter().by_ref(){
        println!("The val {:?}", i);
    }

     println!("{:?}", x);



}