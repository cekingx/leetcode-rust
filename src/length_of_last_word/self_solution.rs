pub struct Solution;

/// # Big O Notation
///
/// time complexity adalah O(n) karena hanya menggunakan sebuah loop
/// sesuai panjang string
///
/// space complexity adalah O(1) karena hanya menggunakan dua buah variable
/// untuk menyimpan angka
///
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let space_char_value = (' ') as i32;
        let mut size = 0;
        for (index, value) in s.chars().enumerate() {
            if value as i32 != space_char_value {
                size += 1;
            }

            if index > 0 {
                if value as i32 != space_char_value && s.chars().nth(index - 1).unwrap() as i32 == space_char_value {
                   size = 1;
                }
            }
        }
        return size;
    }
}