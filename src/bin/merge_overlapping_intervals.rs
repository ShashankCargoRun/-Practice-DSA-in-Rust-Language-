/* Merge Overlapping Intervals
Given a list of intervals, merge all overlapping intervals.
Example:
Input:
[[1,3], [2,6], [8,10], [9,12]]

Output:
[[1,6], [8,12]]
Concepts: Vec, Sorting, Intervals

*/
// cargo run --bin merge_overlapping_intervals



fn main() {
    let mut intervals = vec![[1, 3], [2, 6], [8, 10], [9, 12]];

    // Sort intervals by starting value
    intervals.sort_by_key(|interval| interval[0]);

    let mut result: Vec<[i32; 2]> = Vec::new();

    for interval in intervals {
        if result.is_empty() {
            result.push(interval);
        } else {
            let last = result.last_mut().unwrap();

            // Check if intervals are overlapping
            if interval[0] <= last[1] {
                // Merge the intervals
                if interval[1] > last[1] {
                    last[1] = interval[1];
                }
            } else {
                result.push(interval);
            }
        }
    }

    println!("Merged intervals: {:?}", result);
}

