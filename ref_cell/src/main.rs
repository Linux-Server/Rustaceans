fn main() {
    let x = String::from("Bik");
    x.send();
    let per = Person{name:x};
    per.killer();
}


trait Summary{
    fn send(&self){
        println!("Im everywhere");
    }
}

struct Person<T>{
    name:T
}

impl<T> Person<T> where T:Summary{
    fn killer(&self){
        println!("her its is");
    }
}

impl Summary for String{}