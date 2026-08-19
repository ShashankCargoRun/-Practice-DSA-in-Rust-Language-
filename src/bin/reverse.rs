// Given an array:

//arr = [10, 20, 30, 40, 50]

//Question: Array ko reverse karo.

//Expected output:

//[50, 40, 30, 20, 10]

// cargo run --bin reverse

/*
fn main() {
    let arr = [10, 20, 30, 40, 50];

    let mut rev = arr;
    rev.reverse();

    println!("{:?}", rev);
}

*/

// Without built-in method

fn main() {
    let mut arr = [10, 20, 30, 40, 50];

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        arr.swap(left, right);
        left += 1;
        right -= 1;
    }

    println!("{:?}", arr);
}