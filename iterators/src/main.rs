/*
Iterators pattern allow you to perform some task on sequence of item in turn
- iterators are lazy means it have no effect until u call method on them
-

*/

fn main() {
    let v1 = vec![1,2,3,4,5,6];
    let res: Vec<i32> = v1.iter().map(|x|x+1).collect();

    println!("The collect method: {:?}", res);
    
}
