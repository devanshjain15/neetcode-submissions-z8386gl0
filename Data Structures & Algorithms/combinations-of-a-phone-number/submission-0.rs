impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
       let digits = digits.chars().collect(); 
       let map: [&[char]; 10] = [
            &[],
            &[],
            &['a', 'b', 'c'],
            &['d', 'e', 'f'],
            &['g', 'h', 'i'],
            &['j', 'k', 'l'],
            &['m', 'n', 'o'],
            &['p', 'q', 'r', 's'],
            &['t', 'u', 'v'],
            &['w', 'x', 'y', 'z'],
        ];
       let mut result = Vec::new(); 
       Self::helper(&mut String::new(), 0, &map, &digits, &mut result);
       result
    }

    fn helper(comb: &mut String, cur: usize, map: &[&[char]], digits: &Vec<char>, result: &mut Vec<String>) {
        if cur >= digits.len() {
            if !comb.is_empty() { 
                result.push(comb.clone()); 
            } 
            return; 
        }

        let chars = map[digits[cur].to_digit(10).unwrap() as usize]; 

        for i in 0..chars.len() { 
            comb.push(chars[i]); 
            Self::helper(comb, cur + 1, map, digits, result); 
            comb.pop(); 
        }
    }
}
