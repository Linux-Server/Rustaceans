fn main(){

    let msg = Message::Quit;
    match msg{
        Message::Quit => println!("Its a quit situation"),
        Message::Move { x, y } => println!("Its a move"),
        Message::Write(val) =>  println!("Its a string"),
        _ => ()

    }

    let msg = Message::ChangeColor(Color::Hsv(10, 20, 30));

    match msg{
        Message::ChangeColor(Color::Hsv(x, y, z)) =>{
            println!("The vals are {x}, {y},{z}");
        }
        _ => ()
            
    }

}

enum Message{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),

}

enum Color{
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}