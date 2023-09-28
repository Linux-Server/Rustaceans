fn main() {
    println!("Hello, world!");

    let mut x =10;

    let y  = &x as *const i32;
    let z = &mut x as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;


    unsafe{
      println!("the value of {:?}", *z);

    }


}
