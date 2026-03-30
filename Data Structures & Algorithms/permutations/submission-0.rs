impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::helper(0,&nums)
    }

    fn helper(start: usize, nums: &Vec<i32>) -> Vec<Vec<i32>> { 
        if start >= nums.len() { 
            return vec![vec![]]; 
        }

        let temp = Self::helper(start + 1, nums); 
        let mut result = Vec::new(); 
        for p in temp {  
            let n = p.len(); 
            for i in 0..=n { 
                let mut expanded = p.clone(); 
                expanded.insert(i, nums[start]);                
                result.push(expanded); 
            }
        }

        return result; 
    }
}
