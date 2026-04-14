use std::cmp::{min, max}; 

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals; 
        intervals.sort_unstable(); 
        let n = intervals.len(); 
        let mut l_o = intervals[0][0]; 
        let mut r_o = intervals[0][1]; 

        let mut ans = Vec::new(); 

        for i in 1..n { 
            let (l_i, r_i) = (intervals[i][0], intervals[i][1]); 
            if l_i <= r_o { 
                l_o = min(l_o, l_i); 
                r_o = max(r_o, r_i); 
            } else { 
                ans.push(vec![l_o, r_o]); 
                l_o = l_i; 
                r_o = r_i; 
            }

        }
        ans.push(vec![l_o, r_o]);

        ans
    }
}