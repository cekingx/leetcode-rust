mod first_occurence_in_string;
use first_occurence_in_string::sliding_window::Solution;

fn main() {
    let result = Solution::str_str(String::from("abc"), String::from("c"));
    println!("result: {}", result);
}