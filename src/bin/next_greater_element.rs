/*   Given an array of integers, 
     for every element find the first greater element on its right side.

     Example:

     Input:  [4, 5, 2, 10, 8]

     Output: [5, 10, 10, -1, -1]

     */
     

     fn main() {
    let nums = vec![4, 5, 2, 10, 8];

    let mut result = vec![-1; nums.len()];
    let mut stack = Vec::new();

    for i in 0..nums.len() {
        while let Some(&j) = stack.last() {
            if nums[i] > nums[j] {
                result[j] = nums[i];
                stack.pop();
            } else {
                break;
            }
        }

        stack.push(i);
    }

    println!("{:?}", result);
}
    