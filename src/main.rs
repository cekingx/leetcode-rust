mod valid_parentheses;
use valid_parentheses::self_solution::Solution;

fn main() {
    println!("{}", Solution::is_valid(String::from("((()")));
}