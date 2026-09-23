fn factorial(n: u32) -> u32 {
    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }

    result
}

fn main() {
    let n = 5;
    let result = factorial(n);

    println!("The factorial of {} is {}", n, result);
}