fn main(){
    let x=10;  //x can hold max of i32 - single

    let a = Message::Quit;

    // let b = List::Cons(10,List::Cons(20,List::Cons(30,List::Nil)));
} 


enum Message{
    Quit, // default space
    Contact{x:i32,y:i32}, // i32 +32
    Exit(String) // string

}


enum List{
    Cons(i32, Box<List>), // i32 + List  
    Nil
}