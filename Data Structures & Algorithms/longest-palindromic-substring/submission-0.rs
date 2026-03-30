impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect(); 
        let n = chars.len(); 
        let mut res = 0; 
        let mut res_len = 1; 
        for i in 0..n { 
            let i = i as i32; 
            let len = Self::expand(i, i, &chars).max(Self::expand(i, i + 1, &chars)); 

            if len > res_len { 
                res_len = len; 
                res = i as usize - (len - 1) / 2; 
            }
        }

        chars[res..res + res_len].iter().collect()
    }

    fn expand(mut l: i32, mut r: i32, chars: &Vec<char>) -> usize { 
        let n = chars.len() as i32; 
        while l >= 0 && r < n && chars[l as usize] == chars[r as usize] { 
            l -= 1; 
            r += 1; 
        }

        (r - l - 1) as usize
    }
}
