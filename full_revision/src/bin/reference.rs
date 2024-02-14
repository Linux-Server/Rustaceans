fn main(){
    println!("welcome to Reference and borrowing");
    let mut x = String::from("Hello");
    mutable_borrow(&mut x);
    borrows(&x);
    println!("the real string is : {:?}", x);


}

fn borrows(data:&String){
    println!("The borrowed value is  : {:?}", data);
}

fn mutable_borrow(data:&mut String){
    data.push_str(" World!")
}