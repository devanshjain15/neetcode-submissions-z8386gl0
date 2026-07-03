class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        return f(0, nums)

def f(i: int, nums: List[int]): 
    if i == len(nums): 
        return [[]]

    temp = []
    permutes = f(i + 1, nums)
    for p in permutes: 
        for j in range(len(p) + 1): 
            p_copy = p.copy()
            p_copy.insert(j, nums[i])
            temp.append(p_copy)
        
    return temp
    

        