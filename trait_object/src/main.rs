fn main() {
    println!("welcome to trait object");
    let books = vec![Box::new(1), Box::new(true)];
    // let book_one =  Book{name:books };
}


struct Book{
    name: Vec<Box<dyn Color>>
}


trait Color{
    fn color(&self){
        println!("The color is undefine");
    }
}

impl Color for i32{}
impl Color for bool{}


// impl<T> Book<T>{
//     fn new(&self){
//         Book{
//             name
//         }
//     }
// }
