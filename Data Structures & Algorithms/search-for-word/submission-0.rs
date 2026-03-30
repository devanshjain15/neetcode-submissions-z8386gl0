impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board; 
        let chars: Vec<char> = word.chars().collect();


        for i in 0..board.len() { 
            for j in 0..board[0].len() { 
                if board[i][j] == chars[0] && Self::dfs(i as i32, j as i32, &mut board, 1, &chars) { 
                    return true; 
                }
            }
        }

        false
    }

    fn dfs(r: i32, c: i32, board: &mut Vec<Vec<char>>, next_char_idx: i32, chars: &Vec<char>) -> bool { 
        if next_char_idx as usize >= chars.len() { 
            return true; 
        }

        let temp = board[r as usize][c as usize]; 
        board[r as usize][c as usize] = '#';

        let offset_r = [0, 0, -1, 1];
        let offset_c = [-1, 1, 0, 0]; 

        for i in 0..4 {
            let new_r = r + offset_r[i]; 
            let new_c = c + offset_c[i]; 

            if new_r >= 0 && new_c >= 0 && 
                (new_r as usize) < board.len() && 
                (new_c as usize) < board[0].len() &&
                board[new_r as usize][new_c as usize] == chars[next_char_idx as usize] && 
                Self::dfs(new_r, new_c, board, next_char_idx + 1, chars) {
                    return true; 
            }
            
        }

        board[r as usize][c as usize] = temp;  

        false
    }
}
