pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let m = needle.len();
        let n = haystack.len();
        if m > n {
            return -1;
        }

        for window_start in 0..=(n-m) {
            for i in 0..m {
                if needle.chars().nth(i).unwrap() != haystack.chars().nth(window_start + i).unwrap() {
                    break;
                }

                if i == (m-1) {
                    return window_start as i32;
                }
            }
        }

        return -1;
    }
}