pub struct Solution;

/// # Big O Notation
/// time complexity O(n) karena menggunakan sebuah loop
///
/// space complexity O(1) karena menggunakan sebuah variabel tambahan
/// yaitu `insert_index`
///
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut insert_index = 1;
        for (index, _) in nums.clone().iter().enumerate() {
            if index == 0 {
                continue;
            }

            if nums[index-1] != nums[index] {
                nums[insert_index] = nums[index];
                insert_index += 1;
            }
        }
        return insert_index as i32;
    }
}