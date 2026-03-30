use std::collections::VecDeque; 

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid; 
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new(); 
        let m = grid.len();
        let n = grid[0].len(); 

        for i in 0..m { 
            for j in 0..n { 
                if grid[i][j] == 2 { 
                    queue.push_back((i as i32, j as i32)); 
                }
            }
        }

        let mut minutes = 0; 
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)]; 
        while !queue.is_empty() {
            for _ in 0..queue.len() { 
                let (r, c) = queue.pop_front().unwrap(); 
                for &(dr, dc) in offsets.iter() { 
                    let new_r = r + dr; 
                    let new_c = c + dc; 

                    if new_r < 0 || new_c < 0 { 
                        continue; 
                    }

                    let new_r = new_r as usize; 
                    let new_c = new_c as usize; 

                    if new_r >= m ||
                       new_c >= n || 
                       grid[new_r][new_c] != 1 { 
                        continue; 
                    }

                    grid[new_r][new_c] = 2;
                    queue.push_back((new_r as i32, new_c as i32)); 
                }
            }

            if !queue.is_empty() { 
                minutes += 1; 
            }
        }

        for i in 0..m { 
            for j in 0..n { 
                if grid[i][j] == 1 { 
                    return -1; 
                }
            }
        }

        minutes
    }
}
