
/*
1. Fahrenheit to Celsius
2. Celsius to Fahrenheit

Enter choice: 1
Enter temperature: 100

Celsius: 37.78
*/

use std::io;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    println!("Enter choice:");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: u32 = choice.trim().parse().expect("Please enter 1 or 2");

    println!("Enter temperature:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let temperature: f64 = input.trim().parse().expect("Please enter a number");

    if choice == 1 {
        let result = fahrenheit_to_celsius(temperature);
        println!("Celsius: {:.2}", result);
    } else if choice == 2 {
        let result = celsius_to_fahrenheit(temperature);
        println!("Fahrenheit: {:.2}", result);
    } else {
        println!("Invalid choice!");
    }
}



