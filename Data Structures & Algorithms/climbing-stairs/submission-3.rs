use std::collections::HashMap; 

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        Self::helper(0, n, &mut HashMap::new())
    }

    fn helper(cur: i32, n: i32, cached: &mut HashMap<i32, i32>) -> i32 { 
        if cur == n { 
            return 1; 
        }

        if cached.contains_key(&cur) {
            return *cached.get(&cur).unwrap();
        }

        let mut count = 0; 
        if cur + 1 <= n { 
            count += Self::helper(cur + 1, n, cached); 
        }

        if cur + 2 <= n { 
            count += Self::helper(cur + 2, n, cached); 
        }

        cached.insert(cur, count); 

        count
    }
}
