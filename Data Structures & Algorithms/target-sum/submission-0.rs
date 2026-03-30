impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        Self::helper(0, 0, target, nums.len() as i32, &nums, &mut HashMap::new())
    }

    pub fn helper(i: i32, sum: i32, target: i32, n: i32, nums: &Vec<i32>, cache: &mut HashMap<(i32, i32), i32>) -> i32 { 
        if i == n { 
            return if sum == target { 1 } else { 0 }; 
        }

        if cache.contains_key(&(i, sum)) { 
            return *cache.get(&(i, sum)).unwrap(); 
        }

        let mut ways = 0; 

        ways += Self::helper(i + 1, sum + nums[i as usize], target, n, nums, cache); 
        ways += Self::helper(i + 1, sum - nums[i as usize], target, n, nums, cache); 

        cache.insert((i, sum), ways); 
        ways
    }
}
