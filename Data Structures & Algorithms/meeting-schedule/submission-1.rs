/**
 * Definition of Interval:
 * #[derive(Debug, Clone)]
 * pub struct Interval {
 *     pub start: i32,
 *     pub end: i32,
 * }
 *
 * impl Interval {
 *     pub fn new(start: i32, end: i32) -> Self {
 *         Interval { start, end }
 *     }
 * }
 */

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Interval>) -> bool {
        let n = intervals.len(); 
        if n != 0 { 
            let mut intervals = intervals; 
            intervals.sort_by_key(|x| x.start); 
            let (mut l_o, mut r_o) = (intervals[0].start, intervals[0].end); 
            for i in 1..intervals.len() { 
                let (l_i, r_i) = (intervals[i].start, intervals[i].end); 
                if l_i < r_o { 
                    return false; 
                } else { 
                    l_o = l_i; 
                    r_o = r_i; 
                }
            }
        }

        true
    }
}
