fn main(){
    let mut x = [
        Rectangle{width:19, height:22},
        Rectangle{width:33, height:88},
        Rectangle{width:44, height:77}
    ];
    
    x.sort_by_key(|r| r.height);
    println!("the keys are {:?}", x);
}

#[derive(Debug)]
struct Rectangle{
    width: i32,
    height: i32
}