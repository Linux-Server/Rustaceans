fn main(){

    let msg = Message::ChangeColor(10, 20, 30);

    match msg{
        Message::Quit => println!("Its a quit situation"),
        Message::Move { x, y } => println!("Its a move"),
        Message::ChangeColor(r, g, b) => println!("Ist a rgb"),
        Message::Write(val) =>  println!("Its a string")
    }

}

enum Message{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),

}