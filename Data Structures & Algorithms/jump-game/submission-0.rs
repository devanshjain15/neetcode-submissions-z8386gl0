impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len() as i32; 
        let mut max_reach = 0; 
        for i in 0..n { 
            if i > max_reach { 
                return false; 
            }
            max_reach = max_reach.max(i + nums[i as usize]); 
        }

        true
    }
}