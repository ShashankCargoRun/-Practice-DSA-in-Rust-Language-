fn count_even(nums: Vec<i32>) -> i32 {
    let mut count = 0;

    for num in nums {
        if num % 2 == 0 {
            count += 1;
        }
    }

    count
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    let result = count_even(nums);

    println!("Number of even elements: {}", result);
}