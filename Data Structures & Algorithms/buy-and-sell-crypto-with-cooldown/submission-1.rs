use std::collections::HashMap; 

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // buy => decision = true;
        // sell => decision = false; 
        let n = prices.len(); 
        let mut dp = vec![vec![0;2]; n + 2];

        for i in (0..n).rev() { 
            dp[i][0] = dp[i + 1][0].max(dp[i + 1][1] - prices[i]); 
            dp[i][1] = dp[i + 1][1].max(dp[i + 2][0] + prices[i]);
        } 

        dp[0][0]
    }
}
