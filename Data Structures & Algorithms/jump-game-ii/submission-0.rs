impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32; 
        let mut jumps = 0; 
        let mut end = 0;
        let mut farthest = 0; 

        for i in 0..n-1 { 
            farthest = farthest.max(i + nums[i as usize]); 

            if i == end { 
                jumps += 1; 
                end = farthest;
            }
        }

        jumps
    }
}
