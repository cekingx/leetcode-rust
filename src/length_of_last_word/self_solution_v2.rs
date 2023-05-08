pub struct Solution;

/// # Big O Notation
///
/// time complexity adalah O(n) karena menggunakan sebuah loop
/// namun lebih efisien karena array di balik sehingga tidak perlu melakukan perbandingan
/// terhadap semua elemen array
///
/// space complexity adalah O(1) karena hanya menggunakan dua buah variable
/// untuk menyimpan angka
///
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut size = 0;
        let space_char_value = (' ') as i32;

        for (index, value) in s.chars().rev().enumerate() {
            if value as i32 != space_char_value {
                size += 1;
            }

            if let Some(val) = s.chars().rev().nth(index + 1) {
                if value as i32 != space_char_value && val as i32 == space_char_value {
                    break;
                }
            }
        }

        return size;
    }
}