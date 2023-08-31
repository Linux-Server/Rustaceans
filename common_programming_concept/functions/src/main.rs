fn main() {
    println!("Hello, functions!");
    let x = [1,2,3,4,5];
    another(x);
}

fn another(array: [i32]){
    println!("Calling another func {:?}", array);
}
