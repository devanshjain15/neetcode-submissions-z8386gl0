impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let m = intervals.len(); 
        let n = queries.len(); 
        let mut ans = Vec::new(); 
        for j in 0..n { 
            let q = queries[j]; 
            let mut l = i32::MAX; 
            for i in 0..m { 
                let interval = &intervals[i];
                if interval[0] <= q && q <= interval[1] { 
                    l = l.min(interval[1] - interval[0] + 1); 
                }
            }
            if l == i32::MAX { 
                l = -1; 
            }
            ans.push(l); 
        }

        ans
    }
}
