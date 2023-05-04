mod search_insert_position;
use search_insert_position::self_solution::Solution;

fn main() {
    let result = Solution::search_insert(vec![1,3,5,6], 5);
    println!("{}", result);
}