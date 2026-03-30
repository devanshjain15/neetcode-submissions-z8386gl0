use std::collections::{BinaryHeap, HashSet}; 
use std::cmp::Reverse; 

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len(); 
        let n = grid[0].len(); 
        let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new(); 
        let mut visited: HashSet<(usize, usize)> = HashSet::new(); 
        let offsets = [(0, 1), (0, -1), (1, 0), (-1, 0)]; 
        
        heap.push((Reverse(grid[0][0]), 0, 0)); 
        while let Some((Reverse(t), r, c)) = heap.pop() { 
            if r == m - 1 && c == n - 1 { 
                return t; 
            }

            visited.insert((r, c)); 

            for i in 0..4 { 
                let new_r = r as i32 + offsets[i].0; 
                let new_c = c as i32 + offsets[i].1; 

                if new_r < 0 || 
                   new_c < 0 { 
                    continue; 
                   }
                
                let new_r = new_r as usize; 
                let new_c = new_c as usize; 

                if (new_r) >= m ||
                   (new_c) >= n ||
                   visited.contains(&(new_r, new_c)) { 
                    continue; 
                }

                heap.push((Reverse(t.max(grid[new_r][new_c])), new_r, new_c)); 
            }

        }

        0
    }
}
