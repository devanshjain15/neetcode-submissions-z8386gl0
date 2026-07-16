class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        colSet = set()
        positiveSet = set() # r + c
        negativeSet = set() # r - c
        board = [['.' for _ in range(n)] for _ in range(n)]

        result = []

        def f(r: int):
            if r ==  n: 
                temp = []
                for row in board: 
                    temp.append("".join(row))

                result.append(temp)
                return

            for c in range(n):
                if c in colSet or (r + c) in positiveSet or (r - c) in negativeSet: 
                    continue

                colSet.add(c)
                positiveSet.add(r + c)
                negativeSet.add(r - c)
                board[r][c] = 'Q'

                f(r + 1)

                colSet.remove(c)
                positiveSet.remove(r + c)
                negativeSet.remove(r - c)
                board[r][c] = '.'

        f(0)
        return result
            
