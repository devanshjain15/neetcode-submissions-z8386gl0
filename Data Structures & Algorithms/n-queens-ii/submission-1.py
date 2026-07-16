class Solution:
    def totalNQueens(self, n: int) -> int:
        colSet = set()
        positiveSet = set() # r + c
        negativeSet = set() # r - c
    
        result = 0

        def f(r: int):
            nonlocal result
            if r ==  n: 
                result += 1
                return

            for c in range(n):
                if c in colSet or (r + c) in positiveSet or (r - c) in negativeSet: 
                    continue

                colSet.add(c)
                positiveSet.add(r + c)
                negativeSet.add(r - c)

                f(r + 1)

                colSet.remove(c)
                positiveSet.remove(r + c)
                negativeSet.remove(r - c)

        f(0)
        return result
            
