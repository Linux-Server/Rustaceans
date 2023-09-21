fn main(){
    let x = Some(10);
    let y:Option<i32> = None;

    match (x,y){
        (Some(a), Some(b)) => println!("The vals are {a}, {b}"),
        _ => println!("Nothing found")
    }

    let z = (1,2,3,4,5,6);

    match z{
        (q,w,e,r,t,y)=> println!("The first value is {q}"),
        _ => println!("Nothing found")
    }

    let s = Some(String::from("sam"));

    if let Some(_val) = s{
        println!("The value is")
    }
    // println!("The s val is {:?}", s); // s is moved

    let point_one = Point{x:12, y:55, z:88};

    match point_one{
        Point { y , ..} => println!("The y value is {:?}", y),
        _ => println!("nothing from point")
    }


    let a = (1,2,3,4,5);

    match a {
        (first, .., last) => println!("Its a {first} and {last}"),
        _ => println!("Not a num")
        
    }


}


struct Point{
    x:i32,
    y:i32,
    z:i32
}