use std::collections::HashMap; 

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len(); 
        let mut m1 = 0; 
        let mut m2 = 0; 

        for i in 0..n { 
            if i == 0 { 
                m1 = nums[0]; 
                continue; 
            }
            if i == 1 { 
                m2 = nums[0].max(nums[1]); 
                continue; 
            }

            let temp = m2; 
            m2 = (m1 + nums[i]).max(m2); 
            m1 = temp; 
        }

        if n == 1 { 
            return m1; 
        }
        m2
    }
}