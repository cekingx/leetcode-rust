pub struct Solution;

/// # Big O Notation
///
/// time complexity: O(N^2) karena menggunakan dua loop
/// space complexity: O(1) karena hanya membutuhkan sebuah variable tambahan
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut occurence_index = 0;
        if haystack.len() < needle.len() {
            return -1;
        }

        for (outer_index, outer) in haystack.chars().enumerate() {
            if outer == needle.chars().nth(0).unwrap() {
                for (inner_index, inner) in needle.chars().enumerate() {
                    occurence_index = outer_index;
                    if outer_index + inner_index > haystack.len() {
                        return -1;
                    }

                    if let Some(val) = haystack.chars().nth(outer_index + inner_index) {
                        if inner != val {
                            occurence_index = 0;
                            break;
                        }

                        if inner_index == needle.len() - 1 {
                            return occurence_index as i32;
                        }
                    }
                }
            }
        }

        return -1;
    }
}