pub mod roman_to_int;
use roman_to_int::roman_to_int::Solution;

fn main() {
    println!("{}", Solution::roman_to_int(String::from("III")));
    println!("{}", Solution::roman_to_int(String::from("IV")));
    println!("{}", Solution::roman_to_int(String::from("IX")));
}