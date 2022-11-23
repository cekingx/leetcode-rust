pub mod add_two_numbers;
use add_two_numbers::add_two_numbers::{ListNode, Solution};

fn main() {
    let mut first_list = Some(Box::new(ListNode{val: 1, next: Some(Box::new(ListNode { val: 2, next: None }))}));
    println!("{:?}", first_list);
}