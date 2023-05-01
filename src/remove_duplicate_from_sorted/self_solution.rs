pub struct Solution;

/// # Big O Notation
/// time complexity O(n) karena menggunakan sebuah loop
///
/// space complexity O(n) karena menggunakan dua buah variabel tambahan
/// untuk menyimpan array unik dan array sisa
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut unique: Vec<i32> = vec![];
        let mut rest: Vec<i32> = vec![];

        for item in nums.iter() {
            if unique.len() == 0 {
                unique.push(item.clone());
            }

            if let Some(value) = unique.last() {
                if value != item {
                    unique.push(item.clone());
                } else {
                    rest.push(0);
                }
            }
        }

        let mut result = unique.clone();
        result.append(&mut rest);
        for (index, _) in nums.clone().iter().enumerate() {
            nums[index] = result[index];
        }

        return unique.len() as i32;
    }
}