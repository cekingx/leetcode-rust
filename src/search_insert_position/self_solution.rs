pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums[0] > target {
            return 0;
        }

        for (index, item) in nums.iter().enumerate() {
            if *item == target {
                return index as i32;
            }

            if index > 0 {
                if nums[index - 1] < target && *item > target {
                    return index as i32;
                }
            }
        }

        return nums.len() as i32;
    }
}