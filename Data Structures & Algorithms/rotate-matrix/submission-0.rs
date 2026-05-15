impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len(); 

        for i in 0..n { 
            for j in i + 1..n { 
                let temp = matrix[i][j]; 
                matrix[i][j] = matrix[j][i]; 
                matrix[j][i] = temp; 
            }
        }

        for row in 0..n { 
            for left in 0..(n/2) {
                let right = n - left - 1;  
                let temp = matrix[row][left]; 
                matrix[row][left] = matrix[row][right]; 
                matrix[row][right] = temp; 
            }
        }
    }
}
