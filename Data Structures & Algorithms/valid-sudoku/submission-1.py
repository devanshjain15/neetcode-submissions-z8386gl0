class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        visitedOnce = {}

        # Checking for row
        for i in range(0, 9): 
            row = board[i]
            for j in range(0, 9): 
                rowElement = row[j]
                if rowElement == ".": continue
                if rowElement in visitedOnce: return False
                visitedOnce[rowElement] = True
            visitedOnce = {}

        # Checking for column
        for i in range(0, 9):
            for j in range(0,9): 
                colElement = board[j][i]
                if colElement == ".": continue
                if colElement in visitedOnce: return False
                visitedOnce[colElement] = True
            visitedOnce = {}
        
        # Checking for inner sub-boxes
        # per = [[] for _ in range(9)]
        per = [[0, 0], [0, 3],[0, 6],[3, 0], [3, 3],[3, 6], [6, 0], [6, 3],[6, 6]]
        
        for i in range(9):
            rowIdx = per[i][0]
            colIdx = per[i][1]
            if not isValidBox(board, rowIdx, colIdx): return False
        return True
#                           0            0
def isValidBox(board, startRowIdx, startColIdx): 
    visitedOnce = {}
    for i in range(3): 
        rowIdx = i + startRowIdx
        for j in range(3): 
            colIdx = j + startColIdx
            element = board[rowIdx][colIdx]
            if element == ".": continue
            if element in visitedOnce: return False
            visitedOnce[element] = True        
    return True
    