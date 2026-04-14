use std::cmp::{max, min}; 

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut new_interval = new_interval; 
        let mut ans = Vec::new(); 
        let mut i = 0; 
        let n = intervals.len(); 
        
        while i < n && intervals[i][1] < new_interval[0] { 
            ans.push(intervals[i].clone()); 
            i += 1; 
        }

        while i < n && intervals[i][0] <= new_interval[1] { 
            new_interval[0] = min(intervals[i][0], new_interval[0]); 
            new_interval[1] = max(intervals[i][1], new_interval[1]); 
            i += 1; 
        }

        ans.push(new_interval); 

        while i < n { 
            ans.push(intervals[i].clone()); 
            i += 1;
        }

        ans
    }
}