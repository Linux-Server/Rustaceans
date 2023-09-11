use std::ops::Deref;

fn main(){
    //This will work
    let x = Box::new(10);
    assert_eq!(10, *x);
    // This wont
    let y = MyBox::new(10);
    assert_eq!(10, *y);


}

struct MyBox(i32);


impl MyBox{
    fn new(target: i32)-> MyBox{
        println!("The new funct called..");
        MyBox(target)
    }

}


impl Deref for MyBox{
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}