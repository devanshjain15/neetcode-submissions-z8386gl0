use std::collections::HashSet; 

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len(); 
        let s_bytes = s.as_bytes(); 
        let word_set: HashSet<String> = word_dict.into_iter().collect(); 
        let mut dp = vec![false; n + 1]; 
        dp[0] = true; 
        for i in 1..=n { 
            for j in 0..i { 
                if dp[j] { 
                    let sub = std::str::from_utf8(&s_bytes[j..i]).unwrap(); 
                    if word_set.contains(sub) { 
                        dp[i] = true; 
                        break; 
                    }
                }
            }
        }

        dp[n]
    }
}