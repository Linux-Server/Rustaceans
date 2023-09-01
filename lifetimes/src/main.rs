fn main() {
    println!("Hello, world!");
    let x = 20;//stack
    let res; 
    {
        let y: i32 = 10;//stack
        res = longest(&x, &y);
        
    }//y invalid

    println!("{res}"); //valdit
 
}

// lifetime is checking wheater a ref is valid till its final usage has been completed
fn longest<'a>(x:&'a i32, y:&'a i32)-> &'a i32{  //
    if x > y{
        println!("X is greater");
        x

    }else{
        println!("Y is greater");
        y
    }
}
