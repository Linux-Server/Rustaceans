use std::collections::HashMap;

fn main() {

    let res = fibonacci(10);
    println!("the fib is {:?}", res);

    let my_vec = vec![1,2,3,4,5,6,7];
    let target = 10;

    let b = two_sum(my_vec,target);

    println!("The target {:?}", b);
}

fn fibonacci(n: u32) -> u128 {
    // Use a and b to store the previous two values in the sequence
    let mut a = 0;
    let mut b = 1;
    for _i in 1..n {
        // As we iterate through, move b's value into a and the new computed
        // value into b.
        let c = a + b;
        a = b;
        b = c;
    }
    b
}


fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    
    //
    let mut distance_table: HashMap<i32, usize> = HashMap::new();

    for (i, current_value) in nums.iter().enumerate() {
        match distance_table.get(&(target - current_value)) {
            Some(j) => return Some((i, *j)),
            _ => distance_table.insert(*current_value, i),
        };
    }

    // No match was found!
    None
}
