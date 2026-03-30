impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len(); 
        if n == 1 { 
            return nums[0]; 
        }

        Self::helper(0, n - 2, &nums).max(Self::helper(1, n - 1, &nums))
    }

    fn helper(start: usize, end: usize, nums: &Vec<i32>) -> i32 { 
        let mut m1 = 0; 
        let mut m2 = 0; 

        for i in start..=end { 
            let temp = (m1 + nums[i]).max(m2); 
            m1 = m2; 
            m2 = temp; 
        }

        m2
    }
}
