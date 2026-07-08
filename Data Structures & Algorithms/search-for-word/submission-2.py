class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        m = len(board)
        n = len(board[0])       
        for i in range(m): 
            for j in range(n): 
                if word[0] == board[i][j]: 
                    res = f(i, j, board, 1, word) 
                    if res == True: 
                        return True
        return False

def f(r: int, c: int, board: List[List[str]], i: int, word: str) -> bool:
    if i == len(word):  
        return True 
        
    char = board[r][c]
    board[r][c] = '#'

    delta = [(0, 1), (1, 0), (-1, 0), (0, -1)]
    for (dr, dc) in delta: 
        new_r = r + dr
        new_c = c + dc
        if (
            0 <= new_r < len(board)
            and 0 <= new_c < len(board[0])
            and board[new_r][new_c] != '#'
            and board[new_r][new_c] == word[i]
        ):
            if f(new_r, new_c, board, i + 1, word):
                board[r][c] = char
                return True

    board[r][c] = char

    return False