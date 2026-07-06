class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates.sort()
        combinations = []
        f(0, target, candidates, [], combinations)
        return combinations

def f(start: int, remaining: int, nums: List[int], curCombination: List[int], combinations: List[List[int]]): 
    if remaining == 0: 
        combinations.append(curCombination.copy()); 

    if remaining < 0: 
        return

    for j in range(start, len(nums)): 
        if j > start and nums[j] == nums[j - 1]: 
            continue
        curCombination.append(nums[j])
        f(j + 1, remaining - nums[j], nums, curCombination, combinations)
        curCombination.pop()