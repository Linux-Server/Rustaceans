fn main(){
    let mut x  = Vec::new();
    x.push(1);
    x.push(2);
    x.push(3);

    while let Some(val) = x.pop(){
        println!("Poping the value {:?}", val);
    }

}