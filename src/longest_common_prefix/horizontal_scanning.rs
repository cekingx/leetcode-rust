pub struct Solution;

/// # Horizontal Scanning
/// prinsipnya adalah membandingkan semua element pada array menggunakan
/// sebuah fungsi yang menerima dua parameter
/// ```
/// LCP(s1..sn) = LCP(LCP(LCP(s1, s2), s2)...sn)
/// ```
///
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = strs[0].clone();
        for str in strs {
            result = Solution::get_common_prefix(&result, &str);
        }
        return result;
    }

    pub fn get_common_prefix(str1: &String, str2: &String) -> String {
        let iteration = if str2.len() > str1.len() { str1.len() } else { str2.len() };

        let mut common_prefix = String::from("");
        for i in 0..iteration {
            if &str1[i..i+1] != &str2[i..i+1] {
                break;
            }
            common_prefix.push_str(&str1[i..i+1]);
        }
        return common_prefix;
    }
}