fn main() {
    // What is string slice
    // data is stored in binary

    let x = "sam"; //string literal  (string slice - bcoz its a borrowed form of str)
    let y = &x[..];


    // Data is stored
    //its an owned value
    let mut z = String::from("hello");
    z.push_str("world");
    z.push('l');

    //Vec vs String
    // String is build around vector by using a wrapper
    let x = String::new();
    let x = Person::new();
    let number = "42";
    let z = Vec::<i32>::new(); // struct Vec<T> {}
    let parsed = number.parse::<i32>(); // fn parse<T>(){...}

    let five = Box::<i32>::new(5);

    let x = &"hello"[0];
    

}


struct Person{
    name: String
}

impl Person{
    fn new()-> Self{
       Self{
         name:"".to_string()
       }
    }
}


fn test4(){

}

fn test2(mut data: &str){
    println!("Im test2 ; {:?}", data);
    data = "ram";
}

fn test1(){
    println!("The revise");
    let mut x = 10;
    x = 20;


    let y = &mut 20;
    *y = 30;


    let words = "qwerty";
    let first_letter = &words[0..2];



    println!("the value is {:?}", first_letter);
}

fn test3(){
    let x = 10;
    let y = &&x;

    let c = "sam";
    let d = &c;

    println!("The exact val is {:?}", y);

    assert_eq!(10, **y);
    assert_eq!("sam", *d);  // &str , str


    let mut x = String::from("sam");
    test2(&mut x);

    println!("After mutation {:?}", x);

    //
    // let mut a = "sam";
    // a = "ram";
    //
    // println!("The a val is {:?}", a);
}