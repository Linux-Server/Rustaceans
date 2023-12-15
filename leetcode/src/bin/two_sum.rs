use std::collections::HashMap;

fn main() {
    let nums = vec![3,4,5];
    let target = 9;
    let result = two_sum(nums,target);
    println!("the two sum is {:?}", result);


}

fn two_sum(nums:Vec<i32>, target:i32)->Vec<i32>{
    //return the index of the nums which will add into a same as target
    // ex: [3,4,5] and target=9  => result = [1,2]
         //3 =>9-3 = 6,0
         //4 =>9-4 = 5,1
    let mut store = HashMap::<i32,i32>::new();
    for (index,val) in nums.iter().enumerate(){
        match store.get(val){
            Some(&real)=>{
                return vec![index as i32, real];
            },
            None => {
                store.insert(target-val, index as i32);
            }
        }
    }

    vec![]

}

fn funny(num:&i32){

}
