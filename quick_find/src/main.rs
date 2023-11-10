#![allow(dead_code, unused_variables)]


fn main() {
   //Quick find of Quick Union dynamic connecticity

    let outs = generate_array(10);
    println!("The vector is {:?}", outs);
    let mut dummy:Vec<usize> = vec![0,1,2,1,4,5,6,7,8,9];
    let status = is_connected(&dummy, 1, 3);
   //  dbg!(status);
   let check_union = union_connect(&mut dummy, 1, 3);
   dbg!(dummy);

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
//O(1) - constant time
fn is_connected(my_array:&Vec<usize>,a:usize, b:usize)-> bool{
   // Is vector index of 1 and 3 have same id (right now its 1 and 3 )
       if my_array[a] == my_array[b]{
         true
       }else{
         false

       }   
      
}

fn union_connect(my_array:&mut Vec<usize>, a:usize, b:usize){
   // union(1,3)
   // vec![0,1,2,3,4,5,6,7,8,9];
    // vec![0,3,2,3,4,5,6,7,8,9];  //0,3
    let aid = my_array[a]; //1
    let bid = my_array[b]; //3

   for (i,val) in my_array.iter().enumerate(){
      
      if my_array[i] == bid {
         my_array[i] = bid;
      }

   }
  

}