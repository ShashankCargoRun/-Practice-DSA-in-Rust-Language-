// largest_num.rs

fn find_largest(nums: Vec<i32>) -> i32 {
    let mut largest = nums[0];

    for num in nums {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn main() {
    let nums = vec![10, 25, 5, 40, 15];

    let result = find_largest(nums);

    println!("Largest number is: {}", result);
}