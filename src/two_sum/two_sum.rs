pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut first_index = 0;
        let mut second_index: usize;

        for _outer in nums.iter() {
            second_index = 0;
            for _inner in nums.iter() {
                if first_index != second_index {
                    let sum = nums[first_index] + nums[second_index];
                    if sum == target {
                        return vec![first_index as i32, second_index as i32]
                    }
                }
                second_index += 1;
            } 
            first_index += 1;
        }

        return vec![0, 0];
    }
}