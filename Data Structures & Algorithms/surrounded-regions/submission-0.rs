impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len(); 
        let n = board[0].len(); 
        for i in 0..m { 
            if board[i][0] == 'O' { 
                Self::dfs(i as i32, 0, board); 
            } 
            if board[i][n - 1] == 'O' {  
                Self::dfs(i as i32, (n as i32) - 1, board); 
            }
        }

        for j in 0..board[0].len() { 
            if board[0][j] == 'O' { 
                Self::dfs(0, j as i32, board); 
            } 
            if board[m - 1][j] == 'O' {  
                Self::dfs((m as i32) - 1, j as i32, board); 
            }
        }

        for i in 0..m { 
            for j in 0..n { 
                if board[i][j] == 'O' { 
                    board[i][j] = 'X'; 
                } else if board[i][j] == '#' { 
                    board[i][j] = 'O'; 
                }
            }
        }
    }

    fn dfs(r: i32, c: i32, board: &mut Vec<Vec<char>>) { 
        board[r as usize][c as usize] = '#'; 

        let offsets = [(-1, 0), (1, 0), (0, 1), (0, -1)];   
        for &(dr, dc) in offsets.iter() { 
            let new_r = r + dr; 
            let new_c = c + dc; 
            if new_r < 0 || new_c < 0 { 
                continue; 
            }

            let new_r = new_r as usize; 
            let new_c = new_c as usize; 

            if new_r >= board.len() || 
               new_c >= board[0].len() ||
               board[new_r][new_c] != 'O' { 
                continue; 
               }

            Self::dfs(new_r as i32, new_c as i32, board); 
        }
    }
}
