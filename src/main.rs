mod length_of_last_word;
use length_of_last_word::self_solution_v2::Solution;

fn main() {
    let result = Solution::length_of_last_word(String::from("   dirga   yasa   "));
    println!("result: {}", result);
}