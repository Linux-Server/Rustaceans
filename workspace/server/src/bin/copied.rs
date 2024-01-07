fn main() {
    println!("The copied");
    let x = vec![1,2,3,4];
    let x_iter = x.iter();

    let x_copied = x_iter.map(|&val| val).collect::<Vec<i32>>();

    println!("The copy is {:?}", x_copied);

}