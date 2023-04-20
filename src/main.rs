mod longest_common_prefix;
use longest_common_prefix::horizontal_scanning::Solution;

fn main() {
    println!("{}", Solution::longest_common_prefix(
        vec![
            String::from("car"),
            String::from("cir")
        ]
    ));
}