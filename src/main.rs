pub mod palindrome_number;
use palindrome_number::palindrome_number::Solution;

fn main() {
    println!("is 121 palindrom: {}", Solution::is_palindrome(121));
    println!("is 1221 palindrom: {}", Solution::is_palindrome(1221));
    println!("is 123 palindrom: {}", Solution::is_palindrome(123));
}