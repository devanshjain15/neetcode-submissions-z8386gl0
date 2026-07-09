class Solution:
    def partition(self, s: str) -> List[List[str]]:
        result = []
        f(0, s, [], result)
        return result

def f(i: int, s: str, curPart, result: List[List[str]]): 
    if i == len(s): 
        result.append(curPart.copy())
        return     
            
    len_s = len(s) 
    for j in range(i, len_s): 
        if is_palindrome(i, j, s): 
            curPart.append(s[i : j + 1])
            f(j + 1, s, curPart, result)
            curPart.pop()



def is_palindrome(left: int, right: int, s: str) -> bool: 
    while left < right: 
        if s[left] != s[right]: 
            return False

        left += 1
        right -= 1
        
    return True