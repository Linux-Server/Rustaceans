fn main() {
    println!("Hello, world!");

    let mut x =10;

    let y  = &x as *const i32;
    let z = &mut x as *mut i32;

    println!("the value of {:?} and {:?}", y,z);

}
