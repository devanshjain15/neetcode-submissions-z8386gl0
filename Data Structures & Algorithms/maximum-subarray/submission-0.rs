impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0]; 
        let mut cur_sum = nums[0]; 
        for i in 1..nums.len() { 
            cur_sum = if cur_sum + nums[i] > nums[i] { 
                cur_sum + nums[i]
            } else { 
                nums[i]
            }; 

            max_sum = max_sum.max(cur_sum); 
        }

        max_sum
    }
}
