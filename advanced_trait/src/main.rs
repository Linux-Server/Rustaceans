fn main() {
    println!("Hello, world!");
    let x =10;
    let y = String::from("hello");
    let res = x.test();
    let res1 = y.test();
    println!("The first id {:?} and second {:?}", res, res1);
}

trait Test<T>{
    fn test(&self)->Option<T>;
}

impl Test<i32> for i32{
    fn test(&self)->Option<i32> {
        Some(10)
    }
}

impl Test<String> for String{
    fn test(&self)->Option<String> {
       Some(format!("killer"))
    }
}

