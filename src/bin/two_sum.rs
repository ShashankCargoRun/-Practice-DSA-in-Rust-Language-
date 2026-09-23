// two_sum

// Two Sum

//Given an array of integers nums and an integer target, 
// return the indices of two numbers such that they add up to target.

// Input: nums = [2, 7, 11, 15], target = 9
// Output: [0, 1]

use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&index) = map.get(&complement) {
            return vec![index, i];
        }

        map.insert(num, i);
    }

    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let result = two_sum(nums, target);

    println!("{:?}", result);
}










