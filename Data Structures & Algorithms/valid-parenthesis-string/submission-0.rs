impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut min_open = 0; 
        let mut max_open = 0; 
        for c in s.chars() { 
            match c { 
                '(' => { 
                    max_open += 1; 
                    min_open += 1; 
                 }, 
                ')' => {
                    max_open -= 1; 
                    min_open -= 1; 
                 }, 
                '*' => {
                    max_open += 1; 
                    min_open -= 1; 
                 }, 
                _ => {}
            }

            if max_open < 0 { 
                return false; 
            }

            min_open = min_open.max(0); 
        }

        min_open == 0
    }
}
