use std::collections::HashSet; 

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut history = HashSet::new(); 
        let mut temp = n; 
        while true { 
            let mut sum = 0; 
            while (temp > 0) { 
                let d = temp % 10; 
                sum += d * d; 
                temp /= 10; 
            }
            if sum == 1 { 
                break; 
            }

            if history.contains(&sum) { 
                return false; 
            }

            history.insert(sum); 
            temp = sum;
        }

        true
    }
}
