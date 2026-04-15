impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals; 
        intervals.sort_by_key(|x| x[1]); 

        let mut ans = 0; 
        let mut prev_end = intervals[0][1]; 
        for i in 1..intervals.len() { 
            if intervals[i][0] < prev_end { 
                ans += 1; 
            } else { 
                prev_end = intervals[i][1];  
            }
        }

        ans
    }
}
