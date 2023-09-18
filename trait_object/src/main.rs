fn main() {
    println!("welcome to trait object");
    // let books: Vec<Box<dyn Color>> = vec![Box::new(1), Box::new(true)]; // annotation required
    // let book_one =  Book{name:books };

    let books = vec![Box::new(10), Box::new(true)];


}


struct Book{
    name: Vec<Box<dyn Color>>
}


trait Color{
    fn color(&self){
        println!("The color is undefine");
    }
}

// impl Color for i32{}
// impl Color for bool{}


// impl<T> Book<T>{
//     fn new(&self){
//         Book{
//             name
//         }
//     }
// }
