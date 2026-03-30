use std::collections::HashMap; 

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        Self::helper(0, 0, m, n, &mut HashMap::new())
    }

    fn helper(r: i32, c: i32, m: i32, n: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 { 
        if r == m - 1 && c == n - 1 { 
            return 1; 
        }

        if cache.contains_key(&(r, c)) { 
            return *cache.get(&(r, c)).unwrap(); 
        }

        let mut paths = 0; 
    
        if c + 1 < n { 
            paths += Self::helper(r, c + 1, m, n, cache); 
        }

        if r + 1 < m { 
            paths += Self::helper(r + 1, c, m, n, cache); 
        } 
        cache.insert((r, c), paths); 
        return paths; 
    }
}
