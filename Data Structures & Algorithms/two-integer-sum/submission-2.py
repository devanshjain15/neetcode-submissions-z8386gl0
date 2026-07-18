class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        comp = {}
        result = []
        for i in range(len(nums)): 
            num = nums[i]
            if num in comp: 
                return [comp[num], i]
            comp[target - num] = i

        return result