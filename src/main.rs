mod remove_element;
use remove_element::two_pointer::Solution;

fn main() {
    // let mut vector = vec![3,2,2,3];
    let mut vector = vec![0,1,2,2,3,0,4,2];
    // let mut vector = vec![1,2,2,2,3];
    let result = Solution::remove_element(&mut vector, 2);
    println!();
    println!("{:?}", vector);
    println!("{:?}", result);
}