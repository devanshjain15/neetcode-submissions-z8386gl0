from collections import deque

class Solution:
    def bfs(self, matrix, startingVertex): 
        nodes = []
        rows, cols = len(matrix), len(matrix[0])
        queue = deque()
        visited = set()

        queue.append(startingVertex)
        visited.add(startingVertex)
        
        neighbours = [[-1,0], [0, -1],[1, 0], [0, 1]]
        
        while(queue):
            vertex = queue.popleft()
            r, c = vertex 
            nodes.append(vertex)
            for dr, dc in neighbours: 
                newR, newC = r + dr, c + dc
                if(min(newR, newC) >= 0 and newR < rows and newC < cols and (newR, newC) not in visited and matrix[newR][newC] == "1"): 
                    visited.add((newR, newC))
                    queue.append((newR, newC))
                
        return nodes

    def numIslands(self, grid: List[List[str]]) -> int:
        numberOfIslands = 0
    
        rows, cols = len(grid), len(grid[0])
        visitedLands = []
        
        for r in range(rows): 
            for c in range(cols): 
                if(grid[r][c] == "0" or (r, c) in visitedLands): continue
                coords = (r, c)
                lands = self.bfs(grid, coords)
                visitedLands.extend(lands)
                numberOfIslands += 1
                
        return numberOfIslands
        