fn main(){
    let x = Some(10);
    let y:Option<i32> = None;

    match (x,y){
        (Some(a), Some(b)) => println!("The vals are {a}, {b}"),
        _ => println!("Nothing found")
    }
}