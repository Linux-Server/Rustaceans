fn main(){
    let person = Person{name:"sachin".to_string()};
    // // bighand
    let Person {name: sam} = person;

    println!("the destruct is {:?}", sam);

    //sugar syntax
    let person1 = Person1{name:"sachin".to_string()};

    let Person1 {name} = person1;


    let point_one = Point{x:10,y:7};

    match point_one{
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

}
#[derive(Debug)]
struct Person{
    name:String
}

#[derive(Debug)]
struct Person1{
    name:String
}

struct Point{
    x:i32,
    y:i32
}