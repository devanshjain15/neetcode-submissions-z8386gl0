impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = nums[0]; 
        for i in 1..nums.len() { 
            result = result ^ nums[i]; 
        }

        result
    }
}
