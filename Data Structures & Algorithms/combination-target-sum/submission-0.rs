impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new(); 
        Self::helper(0, &nums, 0, &mut Vec::new(), &mut result, target);
        result 
    }

    fn helper(start: usize, nums: &Vec<i32>,sum: i32, combination: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, target: i32) { 
        if sum == target { 
            result.push(combination.clone()); 
            return; 
        }

        if sum > target || start >= nums.len() { 
            return; 
        }

        let e = nums[start]; 
        combination.push(e); 
        Self::helper(start, nums, sum + e, combination, result, target);
        
        combination.pop(); 
        Self::helper(start + 1, nums, sum, combination, result, target);
    }
}
