pub mod two_sum;
use two_sum::two_sum::Solution;

fn main() {
    let result = Solution::two_sum(vec![1,2,3], 5);
    println!("[{}, {}]", result[0], result[1]);
}
