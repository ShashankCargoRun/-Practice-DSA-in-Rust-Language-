// Basic DSA Question — Easy
// Example: Input:  [4, 7, 2, 9, 1] || Output: 9


fn main() {
    let array = [4, 7, 2, 9, 1];
    let mut max = array[0];

    for number in array.iter() {
        if *number > max {
            max = *number;
        }
    }
    println!("Maximum: {}", max);

}
