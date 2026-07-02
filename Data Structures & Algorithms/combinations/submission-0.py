class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        combinations = []
        f(1, n, k, [], combinations)
        return combinations 

def f(i: int, n: int, k: int, curCombination: List[int], combinations: List[List[int]]): 
    if len(curCombination) == k: 
        combinations.append(curCombination.copy())
        return 
    if i > n: 
        return 

    curCombination.append(i)
    f(i + 1, n, k, curCombination, combinations)
    curCombination.pop() 
    f(i + 1, n, k, curCombination, combinations)

    