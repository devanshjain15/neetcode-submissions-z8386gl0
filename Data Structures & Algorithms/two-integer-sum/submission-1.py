class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        record = dict(); 
        for i in range(0, len(nums)): 
            n = nums[i]
            if n in record : return [record[n], i]
            record[target - n] = i
        
        