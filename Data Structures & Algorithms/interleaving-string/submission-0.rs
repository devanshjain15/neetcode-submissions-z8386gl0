impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let chars_1: Vec<char> = s1.chars().collect(); 
        let chars_2: Vec<char> = s2.chars().collect(); 
        let chars_3: Vec<char> = s3.chars().collect(); 

        let m = chars_1.len(); 
        let n = chars_2.len(); 
        if m + n != chars_3.len() { 
            return false; 
        }

        let mut dp = vec![vec![false; n + 1]; m + 1]; 
        dp[m][n] = true; 

        for i in (0..=m).rev() { 
            for j in (0..=n).rev() { 
                let k = i + j; 

                if i < m && chars_1[i] == chars_3[k] { 
                    dp[i][j] |= dp[i + 1][j]; 
                }

                if j < n && chars_2[j] == chars_3[k] { 
                    dp[i][j] |= dp[i][j + 1]; 
                }
            }
        }

        dp[0][0]
    }
}