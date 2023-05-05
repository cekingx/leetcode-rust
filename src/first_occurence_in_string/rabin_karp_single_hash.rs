use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let m = needle.len();
        let n = haystack.len();
        if n < m {
            return -1
        }

        let radix: u64 = 26;
        let modulo: u64 = 1000000033;
        let mut max_weight: u64 = 1;

        for i in 0..m {
            max_weight = (max_weight * radix) % modulo;
        }

        let hash_needle = Self::hash_value(needle.clone(), radix, modulo, m as u64);
        let mut hash_hay = 0;

        for window_start in 0..=(n-m) {
            if window_start == 0 {
                hash_hay = Self::hash_value(haystack.clone(), radix, modulo, m as u64);
            } else {
                hash_hay = (
                    (hash_hay * radix) % modulo
                        + (haystack.chars().nth(window_start + m - 1).unwrap() as u64 - ('a') as u64)
                        + modulo
                        - ((haystack.chars().nth(window_start - 1).unwrap() as u64 - ('a') as u64) * max_weight) % modulo
                    ) % modulo;
            }

            if hash_needle == hash_hay {
                for i in 0..m {
                    if needle.chars().nth(i).unwrap() != haystack.chars().nth(i + window_start).unwrap() {
                        break;
                    }

                    if i == m-1 {
                        return window_start as i32;
                    }
                }
            }
        }

        return -1;
    }

    pub fn hash_value(to_hash: String, radix: u64, modulo: u64, m: u64) -> u64 {
        let mut ans = 0;
        let mut factor = 1;

        for i in (0..m).rev() {
            if let Some(value) = to_hash.chars().nth(i as usize) {
                ans +=  (
                    (to_hash.chars().nth(i as usize).unwrap() as u64 - ('a') as u64) * factor
                ) % modulo;
                factor = (factor * radix) % modulo;
            }
        }
        return ans % modulo;
    }
}