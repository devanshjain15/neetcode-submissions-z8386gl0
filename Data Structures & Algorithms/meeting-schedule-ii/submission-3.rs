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
    pub fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
        let n = intervals.len(); 
        let mut starts = Vec::new(); 
        let mut ends = Vec::new(); 
        for it in intervals.iter() { 
            starts.push(it.start); 
            ends.push(it.end); 
        }

        starts.sort_unstable(); 
        ends.sort_unstable(); 

        let mut s = 0; 
        let mut e = 0; 
        let mut rooms = 0; 
        let mut max_rooms = 0; 

        while s < n { 
            if starts[s] < ends[e] { 
                s += 1; 
                rooms += 1; 
                max_rooms = max_rooms.max(rooms); 
            } else { 
                rooms -= 1; 
                e += 1; 
            }

        }

        max_rooms 
    }
}
