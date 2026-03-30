impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n: usize = n as usize; 
        let mut result = Vec::new(); 
        let mut col_set = HashSet::new(); 
        let mut pos_diag_set = HashSet::new(); 
        let mut neg_diag_set = HashSet::new(); 

        let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; n]; 
        Self::helper(0, n as i32, &mut board, &mut col_set, &mut pos_diag_set, &mut neg_diag_set, &mut result); 
        result
    }

    fn helper(r: i32, n: i32, board: &mut Vec<Vec<char>>, col_set: &mut HashSet<i32>, pos_diag_set: &mut HashSet<i32>, neg_diag_set: &mut HashSet<i32>, result: &mut Vec<Vec<String>>) { 
        if r == n {
            let mut temp_vec = Vec::new();  
            for row in board.iter() { 
                let mut temp = String::new(); 
                for c in row.iter() { 
                    temp.push(*c); 
                }
                temp_vec.push(temp); 
            }
            result.push(temp_vec); 
            return; 
        }

        for c in 0..(n as usize){ 
            let pos_diag = r + c as i32; 
            let neg_diag = r - c as i32; 
            if col_set.contains(&(c as i32)) || pos_diag_set.contains(&pos_diag) || neg_diag_set.contains(&neg_diag) { 
                continue; 
            }

            col_set.insert(c as i32); 
            pos_diag_set.insert(pos_diag); 
            neg_diag_set.insert(neg_diag); 
            board[r as usize][c] = 'Q'; 
            Self::helper(r + 1, n, board, col_set, pos_diag_set, neg_diag_set, result); 
            col_set.remove(&(c as i32)); 
            pos_diag_set.remove(&pos_diag); 
            neg_diag_set.remove(&neg_diag); 
            board[r as usize][c] = '.'; 

        }
    }
}
