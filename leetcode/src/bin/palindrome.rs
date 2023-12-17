fn main() {
    let x = 121;
    let reverse_ord = x.to_string().chars().rev().eq(x.to_string().chars());
    println!("The res is {:?}",reverse_ord);

}