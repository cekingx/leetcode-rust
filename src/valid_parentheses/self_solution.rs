pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let parentheses = vec![('(', ')'), ('[', ']'), ('{', '}')];
        let mut parentheses_count = vec![(0,0), (0,0), (0,0)];

        for (index, p) in parentheses.iter().enumerate() {
            for c in s.chars() {
                if c == p.0 {
                    parentheses_count[index].0 += 1;
                }

                if c == p.1 {
                    parentheses_count[index].1 += 1;
                }
            }
        }

        for (open, close) in parentheses_count {
            if open != close {
                return false;
            }
        }

        return true;

    }
}