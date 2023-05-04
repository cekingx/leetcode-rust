mod first_occurence_in_string;
use first_occurence_in_string::self_solution::Solution;

fn main() {
    let result = Solution::str_str(String::from("mississippi"), String::from("sipp"));
    println!("{}", result);
}