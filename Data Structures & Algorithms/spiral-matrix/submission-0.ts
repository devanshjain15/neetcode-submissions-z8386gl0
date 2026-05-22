class Solution {
    /**
     * @param {number[][]} matrix
     * @return {number[]}
     */
    spiralOrder(matrix: number[][]): number[] {
        let m = matrix.length;
        let n = matrix[0].length;
        let result: number[] = [];

        enum Direction {
            Left,
            Right,
            Up,
            Down,
        }

        let dir = Direction.Right;

        let top_wall = 0;
        let right_wall = n - 1;
        let left_wall = 0;
        let bottom_wall = m - 1;

        let i = 0;
        let j = 0;

        while (left_wall <= right_wall && top_wall <= bottom_wall) {
            // left to right
            if (dir === Direction.Right) {
                while (j <= right_wall) {
                    result.push(matrix[i][j]);
                    j++;
                }
                top_wall++;
                i++;
                j--;
                dir = Direction.Down;
            }
            // top to bottom
            if (dir === Direction.Down) {
                while (i <= bottom_wall) {
                    result.push(matrix[i][j]);
                    i++;
                }
                right_wall--;
                i--;
                j--;
                dir = Direction.Left;
            }

            // right to left
            if (dir === Direction.Left) {
                while (j >= left_wall && top_wall <= bottom_wall) {
                    result.push(matrix[i][j]);
                    j--;
                }
                bottom_wall--;
                i--;
                j++;
                dir = Direction.Up;
            }

            // bottom to up
            if (dir === Direction.Up) {
                while (i >= top_wall && left_wall <= right_wall) {
                    result.push(matrix[i][j]);
                    i--;
                }
                left_wall++;
                i++;
                j++;
                dir = Direction.Right;
            }
        }

        return result;
    }
}
