impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums; 
        nums.sort_unstable(); 
        let mut result = Vec::new(); 
        Self::helper(0, &nums, &mut Vec::new(), &mut result); 
        result
    }

    fn helper(start: usize, nums: &Vec<i32>, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) { 
        result.push(path.clone()); 

        for i in start..nums.len() { 
            if i > start && nums[i] == nums[i - 1] { 
                continue; 
            }

            path.push(nums[i]);
            Self::helper(i + 1, nums, path, result);  
            path.pop(); 
        }
    }
}
