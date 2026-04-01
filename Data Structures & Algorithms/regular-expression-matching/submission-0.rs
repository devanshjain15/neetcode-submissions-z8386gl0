impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m = s.len(); 
        let n = p.len(); 

        let s_bytes = s.as_bytes(); 
        let p_bytes = p.as_bytes(); 

        let mut dp = vec![vec![false; n + 1]; m + 1]; 
        dp[m][n] = true; 
        
        for i in (0..=m).rev() { 
            for j in (0..n).rev() { 
                let m = i < m && (s_bytes[i] == p_bytes[j] || p_bytes[j] == b'.'); 
                if j + 1 < n && p_bytes[j + 1] == b'*' { 
                    dp[i][j] = dp[i][j + 2] || (m && dp[i + 1][j]); 
                } else { 
                    dp[i][j] = m && dp[i + 1][j + 1]; 
                }
            }
        }

        dp[0][0]
    }
}