/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]

Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]

Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

*/
use std::collections::HashMap;

fn main() {
    let nums = vec![3,3]; // [0,1]
    let target = 6;
    let mut my_map = HashMap::<i32,i32>::new();

    for (index,val) in nums.iter().enumerate(){
        // println!("{index}, {val}");
        match my_map.get(val){
            Some(data) => {
                println!("The result is {:?} and {:?}", data,index as i32);

            },
            None => {
                my_map.insert(target-val, index as i32);

                // 1st => 9-2= (7,0)
            }
        }
    }

}