use std::vec;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let int_array = Self::get_int_array(s);
        let result = Self::get_total_value(int_array);

        return result;
    }

    pub fn get_int_array(s: String) -> Vec<i32> {
        let mut result = Vec::new();
        for char in s.chars() {
            result.push(Self::get_roman_value(char))
        }

        return result;
    }

    pub fn get_total_value(n: Vec<i32>) -> i32 {
        let mut temp: Vec<i32> = n.clone();
        for (index, _) in n.iter().enumerate() {
            let current = n[index];
            let prev = if index == 0 {n[index]} else {n[index-1]};
            if current > prev {
                temp[index-1] *= -1;
            }
        }

        let mut result = 0;
        for value in temp {
            result += value;
        }
        
        return result;
    }

    pub fn get_roman_value(s: char) -> i32 {
        match s {
            'I' => return 1,
            'V' => return 5,
            'X' => return 10,
            'L' => return 50,
            'C' => return 100,
            'D' => return 500,
            'M' => return 1000,
            _ => return 0
        }
    }
}