impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len(); 

        let mut g_max = i32::MIN; 
        for i in 0..n { 
            let mut cur_max = 1; 
            for j in i..n { 
                cur_max *= nums[j]; 
                g_max = g_max.max(cur_max); 
            }

        }

        g_max
    }
}
