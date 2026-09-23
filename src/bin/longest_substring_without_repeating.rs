/* 1. Longest Subarray Without Repeating Elements

Given a string, find the length of the longest substring 
without repeating characters.
Example: Input:  "abcabcbb"
Output: 3
Explanation: "abc" is the longest substring without duplicate characters.
Concepts: HashMap, Sliding Window, Two Pointers

*/


// cargo run --bin longest_substring_without_repeating

use std::collections::HashMap;

fn main() {
    let s = "abcabcbb";

    let mut map = HashMap::new();
    let mut left = 0;
    let mut max_length = 0;

    for (right, ch) in s.chars().enumerate() {
        if let Some(&index) = map.get(&ch) {
            if index >= left {
                left = index + 1;
            }
        }

        map.insert(ch, right);

        let current_length = right - left + 1;

        if current_length > max_length {
            max_length = current_length;
        }
    }

    println!("Longest substring length: {}", max_length);
}
