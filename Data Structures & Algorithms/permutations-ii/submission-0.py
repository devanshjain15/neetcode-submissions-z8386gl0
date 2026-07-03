class Solution:
    def permuteUnique(self, nums: List[int]) -> List[List[int]]:
        return f(0, nums)


def f(i: int, nums: List[int]): 
    if i == len(nums): 
        return [[]]

    temp = []
    permutes = f(i + 1, nums)
    for p in permutes: 
        p_len = len(p)
        history = set()
        for j in range(p_len + 1): 
            p_copy = p.copy() 
            p_copy.insert(j, nums[i])
            if tuple(p_copy) not in history: 
                temp.append(p_copy.copy()) 
                history.add(tuple(p_copy))

            if j < p_len and p[j] == nums[i]: 
                break

    return temp