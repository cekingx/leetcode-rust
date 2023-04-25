pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let mut parentheses: Vec<char> = vec![];

        for c in s.chars() {
            if Self::is_open(&c) {
                parentheses.push(c.clone());
                continue;
            }

            if parentheses.len() > 0 && Self::is_pair(&parentheses[parentheses.len() - 1], &c) {
                parentheses.pop();
                continue;
            }

            if !Self::is_open(&c) {
                return false;
            }
        }

        if parentheses.len() != 0 {
            return false;
        }

        return true;
    }

    pub fn is_open(c: &char) -> bool {
        if *c == '(' || *c == '[' || *c == '{' {
            return true;
        }

        return false;
    }

    pub fn is_pair(left: &char, right: &char) -> bool {
        if *left == '(' && *right == ')' {
            return true;
        }

        if *left == '[' && *right == ']' {
            return true;
        }

        if *left == '{' && *right == '}' {
            return true;
        }

        return false;
    }
}