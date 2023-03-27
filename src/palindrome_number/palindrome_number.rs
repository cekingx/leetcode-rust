pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut compared_value = [x, 0];
        while compared_value[0] > 0 {
            compared_value[1] = (compared_value[1] * 10) + compared_value[0] % 10;
            compared_value[0] = compared_value[0] / 10;
        }

        if x == compared_value[1] {
            return true;
        }

        return false;
    }
}