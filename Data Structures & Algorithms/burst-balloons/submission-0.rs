impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        // let mut nums = nums; 
        let n = nums.len(); 
        let mut cache = vec![vec![-1; n + 1]; n + 1]; 
        nums.insert(0, 1); 
        nums.push(1); 
        Self::f(1, n as i32, &nums, &mut cache)
    }

    fn f(i: i32, j: i32, nums: &Vec<i32>, cache: &mut Vec<Vec<i32>>) -> i32 { 
        if i > j { 
            return 0; 
        } 

        let i = i as usize; 
        let j = j as usize; 

        if cache[i][j] != -1 { 
            return cache[i][j]; 
        }

        let mut cost = 0; 

        for k in i..=j { 
            cost = cost.max(nums[i - 1] * nums[k] * nums[j + 1] + Self::f(i as i32, k as i32 - 1, nums, cache) + Self::f(k as i32 + 1, j as i32, nums, cache)); 
        }

        cache[i][j] = cost; 
        cost
    }
}