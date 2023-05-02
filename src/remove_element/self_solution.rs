pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let initial_length = nums.len();
        let mut swap_index = nums.len() - 1;
        let mut last_index = nums.len();
        let mut deleted = 0;
        for index in 0..nums.len() {
            println!("arr[{}] = {}", index, nums[index]);
            if last_index == index {
                break;
            }

            if nums[swap_index] == val {
                swap_index -= 1;
                println!("decrease swap index");
                deleted += 1;
                println!("increase deleted");
            }

            if nums[index] == val {
                println!("swap value {} {}", nums[index], nums[swap_index]);
                let tmp = nums[swap_index];
                nums[swap_index] = nums[index];
                nums[index] = tmp;
                println!("swapped value {} {}", nums[index], nums[swap_index]);
                deleted += 1;
                println!("increase deleted");
            }

            last_index -= 1;
            println!("last index: {}", last_index);
            println!();
        }

        println!();
        println!("initial length: {}", initial_length);
        println!("deleted: {}", deleted);
        return (initial_length - deleted) as i32;
    }
}