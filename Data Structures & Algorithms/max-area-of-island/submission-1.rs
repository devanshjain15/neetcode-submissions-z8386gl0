impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid; 
        let m = grid.len(); 
        let n = grid[0].len(); 

        let mut result = 0; 

        for i in 0..m { 
            for j in 0..n { 
                if grid[i][j] != 0 /* && !visited.contains(&(i as i32, j as i32)) */ { 
                    grid[i][j] = 0; 
                    result = result.max(Self::dfs(i as i32, j as i32, &mut grid/* , &mut visited */)); 
                    
                }
            }
        }

        result
    }

    fn dfs(r: i32, c: i32, grid: &mut Vec<Vec<i32>>/* , visited: &mut HashSet<(i32, i32)> */) -> i32 { 
        let row_offset: [i32; 4] = [1, -1, 0, 0]; 
        let col_offset: [i32; 4] = [0, 0, 1, -1]; 

        let mut area = 1; 

        for i in 0..4 { 
            let new_r = r + row_offset[i]; 
            let new_c = c + col_offset[i]; 

            if new_r >= 0 && new_c >= 0 &&
               (new_r as usize) < grid.len() && 
               (new_c as usize) < grid[0].len() && 
               grid[new_r as usize][new_c as usize] != 0/*  && !visited.contains(&(new_r, new_c)) */ { 
                /* visited.insert((new_r, new_c));  */
                grid[new_r as usize][new_c as usize] = 0; 
                area += Self::dfs(new_r, new_c, grid/* , visited */); 
            }
        }

        area
    }
}

