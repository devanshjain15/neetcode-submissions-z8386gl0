use std::collections::VecDeque; 

impl Solution {
    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new(); 
        let m = grid.len(); 
        let n = grid[0].len(); 

        for i in 0..m { 
            for j in 0..n { 
                if grid[i][j] == 0 { 
                    queue.push_back((i as i32, j as i32)); 
                }
            }
        }

        let offsets = [(0, -1), (-1, 0), (0, 1), (1, 0)]; 

        while !queue.is_empty() { 
            for _ in 0..queue.len() { 
                let (r, c) = queue.pop_front().unwrap(); 
                for i in 0..4 { 
                    let offset = offsets[i]; 
                    let mut new_r = r + offset.0; 
                    let mut new_c = c + offset.1; 

                    if new_r < 0 || new_c < 0 {continue;}

                    let new_r = new_r as usize; 
                    let new_c = new_c as usize; 
                    if (new_r) >= m ||
                    (new_c) >= n || 
                    grid[new_r][new_c] != i32::MAX
                    { 
                        continue; 
                    }


                    grid[new_r][new_c] = grid[r as usize][c as usize] + 1; 
                    queue.push_back((new_r as i32, new_c as i32)); 
                }   
            }
        }
    }
}
