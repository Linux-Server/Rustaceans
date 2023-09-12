
fn main(){

    let x:i32= 10;
    another(&x);


}


fn another(data: &u32){
    println!("The value is {:?}", data)
}


trait Deref {
    type Target: ?Sized;

    // Required method
    fn deref(&self) -> &Self::Target;
}


impl Deref for i32{
    type Target =u32;
    fn deref(&self) -> &Self::Target {
        &self
    }

}