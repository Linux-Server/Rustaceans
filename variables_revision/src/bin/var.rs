fn main(){
    println!("hello variables");
    // singed integer with 32 bits / 4 bytes
    let num = 10;
    // string literal or string slice
    let _my_string = "hello";
    // String type
    let my_string = String::from("Hello");
    // Unsigned 32 bit integer
    let num_one: u32 = 122;

    let my_bool = true;
    // Constant value
    const MY_CONSTANT : u32 = 100;

    shadow_example();


}

fn shadow_example(){
    let num = 10;
    let num = 20;
    println!("The num is : {}", num);
}

