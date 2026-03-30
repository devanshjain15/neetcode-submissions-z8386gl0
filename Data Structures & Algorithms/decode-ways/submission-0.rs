use std::collections::HashMap; 

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect(); 
        let n = chars.len() as i32; 
        Self::check(0, n, &chars, &mut HashMap::new())
    }

    fn check(start: i32, n: i32, chars: &Vec<char>, cache: &mut HashMap<i32, i32>) -> i32 { 
        if start == n { 
            return 1; 
        }

        if chars[start as usize] == '0' { 
            return 0; 
        }

        if cache.contains_key(&start) { 
            return *cache.get(&start).unwrap(); 
        }

        let mut count =  0; 
        count += Self::check(start + 1, n, chars, cache); 

        // start, start + 1
        if start + 1 < n { 
            let digit = chars[start as usize].to_digit(10).unwrap() * 10 + chars[start as usize + 1].to_digit(10).unwrap(); 
            if digit >= 10 && digit <= 26 { 
                count += Self::check(start + 2, n, chars, cache);
            }
        }

        cache.insert(start, count); 

        count
    } 
}
