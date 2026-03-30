use std::collections::HashMap; 

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len() as i32; 
        let n = matrix[0].len() as i32; 

        let mut dp = vec![vec![-1; n as usize]; m as usize]; 
        let mut ans = 0; 

        for i in 0..m { 
            for j in 0..n { 
                ans = ans.max(Self::dfs(i, j, m, n, &matrix, &mut dp)); 
            }
        }

        ans
    }

    fn dfs(r: i32, c: i32, m: i32, n: i32, matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 { 
        if dp[r as usize][c as usize] != -1 { 
            return dp[r as usize][c as usize]; 
        } 

        let offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)]; 

        let mut l = 0; 

        for offset in offsets.iter() { 
            let new_r = r + offset.0; 
            let new_c = c + offset.1; 

            if new_r < 0 || new_c < 0 || new_r >= m || new_c >= n { 
                continue; 
            }

            if matrix[new_r as usize][new_c as usize] > matrix[r as usize][c as usize] { 
                l = l.max(Self::dfs(new_r, new_c, m, n, matrix, dp)); 
            }
        }

        dp[r as usize][c as usize] = l + 1; 
        l + 1
    }
}
