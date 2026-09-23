/*  Question: Longest Subarray With Sum K
Given an array of integers and an integer k
, find the length of the longest contiguous subarray whose sum is exactly k.

Example:
Input:
arr = [10, 5, 2, 7, 1, 9]
k = 15

Output:
4
*/

// cargo run --bin longest_subarray_sum_k

use std::collections::HashMap;

fn longest_subarray(arr: Vec<i32>, k: i32) -> usize {
    let mut map = HashMap::new();
    let mut sum = 0;
    let mut max_len = 0;

    for i in 0..arr.len() {
        sum += arr[i];

        if sum == k {
            max_len = i + 1;
        }

        if let Some(&index) = map.get(&(sum - k)) {
            max_len = max_len.max(i - index);
        }

        map.entry(sum).or_insert(i);
    }

    max_len
}

fn main() {
    let arr = vec![10, 5, 2, 7, 1, 9];
    let k = 15;

    let result = longest_subarray(arr, k);

    println!("Longest subarray length: {}", result);
}



