impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect(); 
        let n = chars.len();

        let mut count = 0; 

        for i in 0..n { 
            let i = i as i32; 
            count += Self::expand(i, i, &chars); 
            count += Self::expand(i, i + 1, &chars); 
        } 

        count
    }

    fn expand(mut l: i32, mut r: i32, chars: &Vec<char>) -> i32 { 
        let mut count = 0; 
        let n = chars.len(); 

        while l >= 0 && r < n as i32 && chars[l as usize] == chars[r as usize] { 
            count += 1; 
            l -= 1; 
            r += 1; 
        }

        count
    }
}
