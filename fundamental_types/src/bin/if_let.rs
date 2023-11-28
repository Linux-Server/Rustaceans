fn main() {
    let x = Some(10);
    let _y: Option<i32> = None;

    while let Some(i) = x {
        println!("the inner value is {:?}", i);
        break;
    }
}
