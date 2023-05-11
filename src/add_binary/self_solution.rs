pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let loop_length = if a.len() > b.len() {
            a.len()
        } else {
            b.len()
        };

        let mut remainder = '0';
        let mut result: Vec<char> = vec![];
        for i in 0..loop_length {
            if let None = a.chars().rev().nth(i) {
                if let Some(value) = b.chars().rev().nth(i) {
                    if remainder != '0' && value != '0' {
                        result.push('0');
                        remainder = '1';
                        continue;
                    }

                    if remainder != '0' && value == '0' {
                        result.push('1');
                        remainder = '0';
                        continue;
                    }

                    result.push(value);
                    continue;
                }
            }

            if let None = b.chars().rev().nth(i) {
                if let Some(value) = a.chars().rev().nth(i) {
                    if remainder != '0' && value != '0' {
                        result.push('0');
                        remainder = '1';
                        continue;
                    }

                    if remainder != '0' && value == '0' {
                        result.push('1');
                        remainder = '0';
                        continue;
                    }

                    result.push(value);
                    continue;
                }
            }

            if let (Some(a_value), Some(b_value)) = (a.chars().rev().nth(i), b.chars().rev().nth(i)) {
                if a_value == '1' && b_value == '1' && remainder == '1' {
                    remainder = '1';
                    result.push('1');
                    continue;
                }

                if a_value == '1' && b_value == '1' {
                    remainder = '1';
                    result.push('0');
                    continue;
                }

                if a_value == '0' && b_value == '0' && remainder == '1' {
                    remainder = '0';
                    result.push('1');
                    continue;
                }

                if a_value == '0' && b_value == '0' {
                    result.push('0');
                    continue;
                }

                if (a_value == '0' && b_value == '1') || (a_value == '1') && (b_value == '0') {
                    if remainder != '0' {
                        remainder = '1';
                        result.push('0');
                        continue;
                    }

                    result.push('1');
                }
            }
        }

        if remainder != '0' {
            result.push(remainder);
        }

        return result.iter().rev().collect::<String>();
    }
}