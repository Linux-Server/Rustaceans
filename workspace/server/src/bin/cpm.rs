fn main() {
    let binding = vec![1,2,3,4];
    let x = binding.iter();
    let binding = vec![1,2,3,4];
    let y = binding.iter();

    let stat = x.cmp(y);
    println!("Th stst is {:?}", stat);


}