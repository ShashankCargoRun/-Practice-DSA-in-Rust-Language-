// DSA Question: Fibonacci using Dynamic Programming
//
// Given a non-negative integer n, find the nth Fibonacci number.
//
// Fibonacci sequence:
// F(0) = 0
// F(1) = 1
// F(n) = F(n - 1) + F(n - 2)
//
// Example:
// Input:  n = 7
// Output: 13
//
// Requirements:
// 1. Use Dynamic Programming.
// 2. Time Complexity: O(n)
// 3. Space Complexity: O(1)
//
// Function:
// fn fibonacci(n: usize) -> u64

fn fibonacci(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut prev = 0u64;
    let mut curr = 1u64;

    for _ in 1..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}