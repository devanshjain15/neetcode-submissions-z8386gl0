class Solution:
    def islandsAndTreasure(self, grid: List[List[int]]) -> None:
        rows, cols  = len(grid), len(grid[0])
        queue = deque()
        visited = set()

        for r in range(rows): 
            for c in range(cols): 
                if grid[r][c] == 0: 
                    queue.append((r, c))
                    visited.add((r, c))

        neighbors = [(0, -1), (0 , 1), (-1, 0), (1, 0)]
        dist = 0
        while len(queue):
            for _ in range(len(queue)): 
                r, c = queue.popleft()
                for dr, dc in neighbors: 
                    newR, newC = r + dr, c + dc
                    if newR < 0 or newC < 0 or newR >= rows or newC >= cols or (newR, newC) in visited or grid[newR][newC] == -1: 
                        continue
                    grid[newR][newC]= dist + 1
                    queue.append((newR, newC))
                    visited.add((newR, newC))
            dist += 1


        
        