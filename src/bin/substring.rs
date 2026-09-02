// Longest Substring Without Repeating Characters 
// Given a string s, find the length of the longest substring without repeating characters.
// Example:
// Input:  s = "abcabcbb"
// Output: 3

use std::collections::HashMap;

fn longest_substring(s: String) -> usize {
    let chars: Vec<char> = s.chars().collect();

    let mut map = HashMap::new();
    let mut left = 0;
    let mut max_len = 0;

    for right in 0..chars.len() {
        let ch = chars[right];

        if let Some(&prev_index) = map.get(&ch) {
            if prev_index >= left {
                left = prev_index + 1;
            }
        }

        map.insert(ch, right);

        let current_len = right - left + 1;
        max_len = max_len.max(current_len);
    }

    max_len
}

fn main() {
    let s = String::from("abcabcbb");

    let result = longest_substring(s);

    println!("{}", result);
}
