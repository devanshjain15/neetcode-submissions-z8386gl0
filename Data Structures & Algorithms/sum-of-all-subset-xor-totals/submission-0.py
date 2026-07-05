class Solution:
    def subsetXORSum(self, nums: List[int]) -> int:
        def dfs(i: int, xor: int) -> int: 
            if i == len(nums):
                return xor

            include = dfs(i + 1, xor ^ nums[i])
            exclude = dfs(i + 1, xor)
            return include + exclude 

        return dfs(0, 0)