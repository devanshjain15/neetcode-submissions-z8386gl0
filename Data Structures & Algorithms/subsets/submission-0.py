class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        curSubset, powerset = [], []
        f(nums, 0, curSubset, powerset)
        return powerset
        

def f(nums: List[int], i: int, curSubset: List[int], powerset: List[List[int]]):
    if i == len(nums): 
        powerset.append(curSubset.copy())
        return
    curSubset.append(nums[i])
    f(nums, i + 1, curSubset, powerset)
    curSubset.pop() 
    f(nums, i + 1, curSubset, powerset)

