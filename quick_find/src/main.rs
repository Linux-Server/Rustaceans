#![allow(dead_code, unused_variables)]


fn main() {
   //Quick find of Quick Union dynamic connecticity

    let outs = generate_array(10);
    println!("The vector is {:?}", outs);
    let dummy:Vec<usize> = vec![0,1,2,3,4,5,6,7,8,9];
    let status = is_connected(outs, 1, 3);
    dbg!(status);

}

fn generate_array(count: usize)-> Vec<usize>{
   let mut my_array = Vec::with_capacity(count);
    for i in 0..count{
      //   println!("the count is {:?}", i);
        my_array.push(i);
    }
    my_array
}

// Is connected (1,3).. ? , Union(1,3).. ? 

fn is_connected(my_array:Vec<usize>,a:usize, b:usize)-> bool{
   // Is vector index of 1 and 3 have same id (right now its 1 and 3 )
      for (index, data) in my_array.iter().enumerate(){
            // 0,0 => 1,1
         return my_array[a] == my_array[b]
            
      }
      false
}