mod plus_one;
use plus_one::self_solution::Solution;

fn main() {
    let result = Solution::plus_one(vec![9,9]);
    println!("result: {:?}", result);
}