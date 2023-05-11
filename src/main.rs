mod add_binary;
use add_binary::self_solution::Solution;

fn main() {
    let result = Solution::add_binary(String::from("110010"), String::from("10111"));
    println!("result: {}", result);
}