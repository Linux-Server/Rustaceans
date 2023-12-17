fn main() {
     let x = Some(10);
     let mut y = x.iter();
     let c = y.next();
    println!("The {:?}", y.next());
    println!("The {:?}", y.next());

    let a = "sam".to_string();
    let b = a.chars();
    for i in b{
        println!("{i}");
    }

    // let z = y.enumerate();
    // println!("Z is : {:?}", z);

    // if let None = y {
    //     println!("No value found");
    // }

    // while let Some(val) = y{
    //     println!("The Some found with {:?}", val);
    // }


}

