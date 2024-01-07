fn main() {
    println!("The copied");
    let x = vec![1,2,3,4];
    let x_iter = x.iter();

    let x_copied = x_iter.map(|&val| val).collect::<Vec<i32>>();

    println!("The copy is {:?}", x_copied);

    // count the number of elements

    println!("The count is {:?}", x_copied.iter().count());
    println!("The last element in vector is {:?}", x_copied.iter().last());

    let x = vec!["sam".to_string(),"ram".to_string()];
    let x_iter = x.iter();
    let y = x_iter.cloned().collect::<Vec<String>>();
    println!("The value is : {:?}", y);
    println!("The original is : {:?}", x);


}