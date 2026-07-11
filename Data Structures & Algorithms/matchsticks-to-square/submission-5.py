class Solution:
    def makesquare(self, matchsticks: List[int]) -> bool:
        perimeter = sum(matchsticks)
        if perimeter % 4 != 0: 
            return False

        max_side_length = perimeter // 4
        matchsticks.sort(reverse=True)
        if matchsticks[0] > max_side_length: 
            return False
        

        return f(0, max_side_length, [0, 0, 0, 0], matchsticks)
        

def f(i: int, max_side_length: int, state: List[int], matchsticks: List[int]) -> bool: 
    if i == len(matchsticks): 
        return True

    for j in range(4): 
        if state[j] + matchsticks[i] <= max_side_length:
            state[j] += matchsticks[i] 
            if f(i + 1, max_side_length, state, matchsticks): 
                return True
            state[j] -= matchsticks[i] 


    return False