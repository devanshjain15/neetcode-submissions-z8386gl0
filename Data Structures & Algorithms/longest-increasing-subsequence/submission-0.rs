impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len(); 
        let mut dp = vec![1; n]; 
        for i in 1..n { 
            for j in 0..i { 
                if nums[j] == nums[i] { 
                    continue; 
                }

                if nums[j] < nums[i] { 
                    dp[i] = dp[i].max(dp[j] + 1); 
                }
            }
        }

        let mut ans = dp[0]; 
        for i in 1..n { 
            ans = ans.max(dp[i]); 
        }

        ans
    }
}
