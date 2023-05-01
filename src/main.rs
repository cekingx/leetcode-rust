mod merge_two_sorted_lists;
use merge_two_sorted_lists::self_solution::{Solution, ListNode};

fn main() {
    let left = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: None
        }))
    }));
    let right = Some(Box::new(ListNode::new(2)));
    println!("{:?}", Solution::merge_two_lists(left, right));
}