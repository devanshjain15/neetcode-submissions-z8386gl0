impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let m = s.len(); 
        let n = t.len(); 

        let s_bytes = s.as_bytes(); 
        let t_bytes = t.as_bytes(); 

        let mut dp = vec![vec![0; n + 1]; m + 1]; 

        for i in 0..=m { 
            dp[i][n] = 1; 
        }

        for i in (0..m).rev() { 
            for j in (0..n).rev() { 
                if s_bytes[i] == t_bytes[j] { 
                    dp[i][j] = dp[i + 1][j + 1] + dp[i + 1][j]; 
                } else { 
                    dp[i][j] = dp[i + 1][j]; 
                }
            }
        }

        dp[0][0]
    }
}