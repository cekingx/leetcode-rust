mod remove_duplicate_from_sorted;
mod longest_common_prefix;
use remove_duplicate_from_sorted::two_indexes::Solution;

fn main() {
    let mut vector = vec![1,2,2,3,3,4];
    let result = Solution::remove_duplicates(&mut vector);
    println!();
    println!("{:?}", vector);
    println!("{:?}", result);
}