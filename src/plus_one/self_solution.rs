pub struct Solution;

/// # Big O Notation
///
/// time complexity adalah O(2N) atau disederhanakan menjadi O(N) karena menggunakan
/// dua loop terpisah
///
/// space complexity adalah O(N) karena ada sebuah variable untuk menyimpan array
///
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut temp_vec: Vec<i32> = vec![];
        let mut left: i32 = 0;
        for (index, value) in digits.iter().enumerate().rev() {
            if index == digits.len() - 1 {
                if value + 1 == 10 {
                    temp_vec.push(0);
                    left = 1;
                } else {
                    temp_vec.push(value + 1);
                }
                continue;
            }

            if value + left == 10 {
                temp_vec.push(0);
                left = 1;
            } else {
                temp_vec.push(value + left);
                left = 0;
            }
        }

        if left != 0 {
            temp_vec.push(left);
        }

        temp_vec.reverse();
        return temp_vec;
    }
}