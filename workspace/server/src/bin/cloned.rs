fn main() {
    println!("the cloned method in iterator");
    let my_vec = vec![1,2,3,4,5];
    let sums = my_vec.iter().sum::<i32>();
    println!("{sums}");
    println!("{:?}", my_vec);

    // consumable iterators
    let x = vec![1,2,3,4];
    let x = x.iter();
    let y = x.sum::<i32>();
    println!("The val is {:?}",y);
    // println!("the iter is {:?}", x);

    //iterator adapters
    let x = vec![1,2,3,4,5];
    let y  = vec!["sam".to_string(),"ram".to_string(), "jam".to_string()];

    let new_x = x.iter().map(|&val| val*3).sum::<i32>();
    let new2 = x.iter().map(|val| val+&10).collect::<Vec<i32>>();
    println!("The new x is {:?}", new2);

    // By ref

    let mut x = vec![1,22,3,4].into_iter();
    let y = x.by_ref().take(2).collect::<Vec<i32>>();
    // println!("the ref sum is {:?}", x);
    println!("the ref sum is {:?}", y);
    dbg!(x);






}