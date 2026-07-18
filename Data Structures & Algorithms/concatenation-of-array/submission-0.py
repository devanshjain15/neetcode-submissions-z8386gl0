class Solution:
    def getConcatenation(self, nums: List[int]) -> List[int]:
        n = len(nums)
        result = [0 for _ in range(2*n)]
        for i in range(n): 
            result[i] = nums[i]
            result[i + n] = nums[i]

        return result