fn main() {
    println!("the leet sum");
    let nums = vec![1, 4, 3, 8, 6, 7, 5];
    let _target = 4;
    for (i, val) in nums.iter().enumerate() {
        println!("The value is {:?} and {:?}", val, i);
    }
}

/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
 */
