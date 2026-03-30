impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new(); 
        let chars: Vec<char> = s.chars().collect(); 
        Self::helper(0, &mut Vec::new(), &chars, &mut result); 
        result
    }

    fn helper(start: usize, partition: &mut Vec<String>, chars: &Vec<char>, result: &mut Vec<Vec<String>>) { 
        if start >= chars.len() { 
            result.push(partition.clone()); 
            return; 
        }

        for i in start..chars.len() { 
            let mut temp = String::new(); 
            for j in start..=i {
                temp.push(chars[j]); 
            }
            if Self::is_palindrome(&temp) { 
                partition.push(temp); 
                Self::helper(i + 1, partition, chars, result); 
                partition.pop(); 
            }
        }
    }

    fn is_palindrome(s: &String) -> bool { 
        let chars: Vec<char> = s.chars().collect(); 
        let mut i = 0; 
        let mut j = chars.len().saturating_sub(1); 
        while i < j { 
            if chars[i] != chars[j] { 
                return false; 
            } 

            i += 1; 
            j -= 1;
        }

        true
    }
}
