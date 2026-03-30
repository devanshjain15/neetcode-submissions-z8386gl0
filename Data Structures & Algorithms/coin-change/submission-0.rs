impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize; 
        let mut dp = vec![i32::MAX; amount + 1]; 
        dp[0] = 0; 

        for cur_amount in 1..=amount { 
            for &coin in coins.iter() { 
                let delta = cur_amount as i32 - coin; 
                if delta < 0 { 
                    continue; 
                }
                let delta = delta as usize; 
                if dp[delta] != i32::MAX { 
                    dp[cur_amount] = dp[cur_amount].min(dp[delta] + 1); 
                }
            }
        }   

        if dp[amount] == i32::MAX { 
            return -1; 
        }

        dp[amount]
    }
}
