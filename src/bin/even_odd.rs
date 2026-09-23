// Count Even and Odd Numbers
// Example: [1, 2, 3, 4, 6] → Even = 3, Odd = 2

fn main () {

     let arr = [1, 2, 3, 4, 6];

     let result = &arr;

     for result in arr.iter() {

         if result % 2 == 0 {
            println!("{:}number is even", result);
        } else {
            println!("{:}number is odd",  result);
        }
    }
}