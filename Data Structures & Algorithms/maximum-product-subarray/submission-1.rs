impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut max_num = nums[0];  
        let mut cur_min = 1; 
        let mut cur_max = 1; 

        for &num in nums.iter() { 
            max_num = max_num.max(num); 
            if num == 0 { 
                cur_min = 1; 
                cur_max = 1; 
                continue; 
            }

            let temp = cur_max * num; 
            cur_max = num.max(temp).max(cur_min * num); 
            cur_min = num.min(temp).min(cur_min * num); 
            res = res.max(cur_max); 
        }

        res.max(max_num)
    }
}
