// valid_parentheses
// Given a string containing only '(', ')', '{', '}', '[', and ']', 
// determine whether the input string is valid.
// Input: "()[]{}"
//Output: true

//Input: "([)]"
// Output: false


fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '{' | '[' => stack.push(ch),

            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }

            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }

            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }

            _ => return false,
        }
    }

    stack.is_empty()
}

fn main() {
    let s = String::from("()[]{}");

    println!("{}", is_valid(s));
}


