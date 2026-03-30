class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        rowTracker, colTracker, boxTracker = [{} for _ in range(9)], [{} for _ in range(9)],[{} for _ in range(9)]

        for i in range(9): 
            for j in range(9): 
                if board[i][j] == ".": continue

                element = board[i][j]
                boxIdx = (i // 3) * 3 + (j // 3)
                if element in rowTracker[i] or element in colTracker[j] or element in boxTracker[boxIdx]: 
                    return False
                
                rowTracker[i][element] = True
                colTracker[j][element] = True
                boxTracker[boxIdx][element] = True
        return True

