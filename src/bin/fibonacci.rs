pub fn nth_fib(term: u64) -> u64 {
     if term == 0 {
         return 0;
     }
 
     let mut prev = 0;
     let mut curr = 1;
 
     for _ in 1..=term {
         let next = prev + curr;
         prev = curr;
         curr = next;
     }
 
     prev
 }
 
 fn main() {
     let term = 7;
     let result = nth_fib(term);
 
     println!("The {}-th Fibonacci number is {}", term, result);
 }