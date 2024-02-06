fn main() {
    println!("Hello, world!");
    // by default variables are immutable
    // inorder to change the data , you should add mut keyword to the variable
    let mut num = 10;
    num =20;
    // Her the value is copied to the copy_ownership function
    copy_ownership(num);
    println!("End of prgm : {:?}", num);

    let my_string = String::from("Sachin");
    // take_ownership(my_string);
    println!("End of everything : {:?}", my_string);


}

fn copy_ownership(num:i32){
    println!("Cpy Ownership : {}", num);
}

fn take_ownership(data: String){
    println!("Take ownership : {:?}", data);
}
