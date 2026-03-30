from collections import deque

class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        rows, cols = len(grid), len(grid[0])
        queue = deque()

        for r in range(rows): 
            for c in range(cols): 
                if grid[r][c] == 2: 
                    queue.append((r, c))
        minute = 0
        if(len(queue) > 0): 
            neighbors = [(0, -1), (0, 1), (1, 0), (-1, 0)]
            while len(queue): 
                for _ in range(len(queue)): 
                    r, c = queue.popleft()
                    for dr, dc in neighbors: 
                        newR, newC = r + dr, c + dc
                        if newR < 0 or newC < 0 or newR >= rows or newC >= cols or grid[newR][newC] != 1: 
                            continue
                        
                        grid[newR][newC] = 2
                        queue.append((newR, newC))

                if (len(queue) == 0): break; 
                minute += 1
                    
        isAllVisited = True
        for r in range(rows): 
            for c in range(cols): 
                if(grid[r][c] == 1): 
                    isAllVisited = False

        if isAllVisited: 
            return minute
        else: return -1
