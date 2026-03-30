from collections import deque

class Solution:
    def bfs(self, matrix, startingVertex): 
        rows, cols = len(matrix), len(matrix[0])
        queue = deque()

        queue.append(startingVertex)
        
        neighbours = [[-1,0], [0, -1],[1, 0], [0, 1]]
        
        while(queue):
            vertex = queue.popleft()
            r, c = vertex 
            for dr, dc in neighbours: 
                newR, newC = r + dr, c + dc
                if(min(newR, newC) >= 0 and newR < rows and newC < cols and matrix[newR][newC] == "1"): 
                    matrix[newR][newC] = "0"
                    queue.append((newR, newC))
                
       

    def numIslands(self, grid: List[List[str]]) -> int:
        numberOfIslands = 0
    
        rows, cols = len(grid), len(grid[0])
        
        for r in range(rows): 
            for c in range(cols): 
                if(grid[r][c] == "0"): continue
                self.bfs(grid, (r, c))
                numberOfIslands += 1
                
        return numberOfIslands
        