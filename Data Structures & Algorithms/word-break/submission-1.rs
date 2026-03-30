impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len(); 
        let s_bytes = s.as_bytes();  
        let mut dp = vec![false; n + 1]; 
        dp[0] = true; 
        for i in 1..=n { 
            if dp[i - 1] { 
                for w in word_dict.iter() { 
                    let w_len = w.len(); 
                    let w_bytes = w.as_bytes(); 
                    if i - 1 + w_len <= n && w_bytes == &s_bytes[i - 1..i - 1 + w_len] { 
                        dp[i - 1 + w_len] = true;
                    }
                }
            }
        }

        dp[n]
    }
}