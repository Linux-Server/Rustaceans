fn main(){
   // Initialize the array
   let mut my_vector = initialize(10);
   println!("the vector is {:?}", my_vector);

   // find wherether p and q are connected
    quick_union(&mut my_vector, 4, 3);
    println!("{:?}",my_vector);
    quick_union(&mut my_vector, 3, 8);
    println!("{:?}",my_vector);
    quick_union(&mut my_vector, 6, 5);
    println!("{:?}",my_vector);
    quick_union(&mut my_vector, 9, 4);
    println!("{:?}",my_vector);
    quick_union(&mut my_vector, 2, 1);
    println!("{:?}",my_vector);
    quick_union(&mut my_vector, 5, 0);
    println!("{:?}",my_vector);
    quick_union(&mut my_vector, 7, 2);
    println!("{:?}",my_vector);
    quick_union(&mut my_vector, 6, 1);
    println!("{:?}",my_vector);
    quick_union(&mut my_vector, 4, 3);
    println!("{:?}",my_vector);


}


fn quick_union(my_array: &mut Vec<usize>, p: usize, q:usize){  // 3, 4
    let i = root(p, my_array);//3 //7
    let j = root(q, my_array);//4
    // 3,4
    my_array[i] = j;

}

fn root(mut i:usize, my_array:&Vec<usize>)->usize{ //3
    
    while(i != my_array[i]){  // i=3 and my_arr[3] = 7   ====> // i=7 and my_arr[7] = 7
        i= my_array[i];   // prev=3 => 7
    }

    return i; // if 3 ==3   =====> 7
}

//0,1,2,3,4,5,6,7
//0,1,2,7,4,5,6,7



fn initialize(num: usize)->Vec<usize>{
    let mut temp_vec:Vec<usize>= Vec::new();
    for i in 0..num{
        temp_vec.push(i);
    }
    temp_vec
}


fn quick_find(my_array: &Vec<usize>,p: usize, q:usize)-> bool{
    return  my_array[p] == my_array[q];
}