mod first_occurence_in_string;
use first_occurence_in_string::rabin_karp_single_hash::Solution;

fn main() {
    let hash = Solution::hash_value(String::from("dirga"), 26, 1000000033, 2);
    println!("hash \"di\": {}", hash);

    let result = Solution::str_str(String::from("ababcaababcaabc"), String::from("ababcaabc"));
    println!("result: {}", result);
}