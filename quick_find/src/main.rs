fn main() {
   //Quick find of Quick Union dynamic connecticity
   let id = [0,1,2,3,4,5,6,7,8,9];

    let outs = generate_array(10);
    println!("The vector is {:?}", outs);

}

fn generate_array(count: usize)-> Vec<usize>{
   let mut my_array = Vec::with_capacity(count);
    for i in 0..count{
      //   println!("the count is {:?}", i);
        my_array.push(i);
    }
    my_array
}
