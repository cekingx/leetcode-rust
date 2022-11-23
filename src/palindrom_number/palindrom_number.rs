pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if !Self::initial_check(x) {
            return false;
        }

        let splitted_string = Self::split_string(x.to_string());
        if splitted_string[0].eq(&splitted_string[1].chars().rev().collect::<String>()) {
            return true;
        }
        return false;
    }

    fn initial_check(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        return true;
    }

    fn split_string(word: String) -> Vec<String> {
        if word.len() % 2 == 1 {
            let half = (word.len() - 1) / 2;
            return vec![word[0..half].to_string(), word[half+1..].to_string()];
        }

        let half = word.len() / 2;
        return vec![word[0..half].to_string(), word[half..].to_string()];
    }
}