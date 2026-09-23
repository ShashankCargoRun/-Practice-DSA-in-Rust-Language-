/* Maximum Subarray Sum
Given an integer array nums, find the contiguous subarray that has the largest sum.
Return the maximum sum.
Example:

Input:  nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4]
Output: 6
*/

// cargo run --bin maximum_subarray_sum


fn main() {

    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

    let mut current_sum = nums[0]; 
    let mut max_sum = nums[0];

    for i in 1..nums.len() {

        current_sum = std::cmp::max(nums[i], current_sum + nums[i]);
        max_sum = std::cmp::max(max_sum, current_sum);

    }


    println!("Maximum subarray sum: {}", max_sum); 
}