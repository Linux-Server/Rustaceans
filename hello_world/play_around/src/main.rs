#[allow(dead_code)]
const OUR_COURSE : &str = "The auto gpt";
fn main() {

    let name ="Sachin";
    println!("{}", name);

    let dynamic_name = "Anju".to_string();
    println!("{:?}", dynamic_name);

    let x = &dynamic_name[..2];

    println!("{:?}", x);


    let mut chars = Vec::with_capacity(10);
    chars.push('e');
    chars.push('l');
    chars.insert(2, 'h');

    let x = chars.pop();

    let b = dbg!(&chars);

 
    println!("The dbg is {:?}", b);

    println!("The chars are {:?}", chars);


}

