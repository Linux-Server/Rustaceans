use std::cell::RefCell;

fn main(){
    let x = Test{data: vec!["sam".to_string()].into()};
    chek(&x);

    println!("ref celll : {:?}", x);



}

#[derive(Debug)]
struct Test{
    data: RefCell<Vec<String>>
}


fn chek(x: &Test){
    x.data.borrow_mut().push("jeee".to_string());
}