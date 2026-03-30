class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        maxArea = 0
        rows = len(grid)
        cols = len(grid[0])
        visitedLand = set()
        lastLandCount = len(visitedLand)

        def dfs(grid, coords, visitedLand): 
            rows = len(grid)
            cols = len(grid[0])
            r, c = coords
            if (r >= 0 and c >= 0 and r < rows and c < cols and grid[r][c] == 1 and coords not in visitedLand):    
                visitedLand.add(coords)
                visitedLand = dfs(grid, (r + 1, c), visitedLand)
                visitedLand = dfs(grid, (r - 1, c), visitedLand)
                visitedLand = dfs(grid, (r, c + 1), visitedLand)
                visitedLand = dfs(grid, (r, c - 1), visitedLand)

            return visitedLand
        

        for r in range(rows):
            for c in range(cols): 
                coords = (r, c)
                if grid[r][c] == 1 and coords not in visitedLand:
                    visitedLand = dfs(grid, coords, visitedLand)
                    print(visitedLand)
                    delta = len(visitedLand) - lastLandCount
                    if(delta > maxArea): maxArea = delta
                    lastLandCount = len(visitedLand)
                

        return maxArea