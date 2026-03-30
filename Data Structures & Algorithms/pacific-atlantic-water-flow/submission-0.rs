impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len(); 
        let n = heights[0].len(); 

        let mut pacific_reachable: Vec<Vec<bool>> = vec![vec![false; n]; m]; 
        let mut atlantic_reachable: Vec<Vec<bool>> = vec![vec![false; n]; m]; 
        // instead of first populating reachable and then checking for others, populate and check for reachability together
        for i in 0..m {
            let i = i as i32; 
            Self::dfs(i, 0, &mut pacific_reachable, &heights);
            Self::dfs(i, (n as i32) - 1, &mut atlantic_reachable, &heights);  
        } 

        for j in 0..n { 
            let j = j as i32;
            Self::dfs(0, j, &mut pacific_reachable, &heights);
            Self::dfs((m as i32) - 1, j, &mut atlantic_reachable, &heights);  
        }

        let mut result = Vec::new(); 

        for i in 0..m { 
            for j in 0..n { 
                if pacific_reachable[i][j] && atlantic_reachable[i][j] { 
                    result.push(vec![i as i32, j as i32]); 
                }
            }
        }
        result
    }

    fn dfs(r: i32, c: i32, reachable: &mut Vec<Vec<bool>>, heights: &Vec<Vec<i32>>) { 
        reachable[r as usize][c as usize] = true; 
        let offsets = [(-1,0), (1, 0), (0, 1), (0, -1)]; 

        for &(dr, dc) in offsets.iter() { 
            let new_r = r + dr; 
            let new_c = c + dc; 

            if new_r < 0 || new_c < 0 { 
                continue; 
            } 

            let new_r = new_r as usize; 
            let new_c = new_c as usize; 
            if new_r >= heights.len() || 
               new_c >= heights[0].len() || 
               reachable[new_r][new_c] || 
               heights[new_r][new_c] < heights[r as usize][c as usize] { 
                continue; 
            }

            Self::dfs(new_r as i32, new_c as i32, reachable, heights); 
        }
    }
}
