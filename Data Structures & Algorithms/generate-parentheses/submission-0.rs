impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new(); 
        Self::helper(0, 0, &mut String::new(), &mut result, n); 
        result
    }

    fn helper(open: i32, close: i32, combination: &mut  String, result: &mut Vec<String>, n: i32) { 
        if open == close && open == n { 
            result.push(combination.clone()); 
        }

        if open < n { 
            combination.push('('); 
            Self::helper(open + 1, close, combination, result, n); 
            combination.pop();  
        }

        if close < open { 
            combination.push(')'); 
            Self::helper(open, close + 1, combination, result, n); 
            combination.pop(); 
        }
    }
}
