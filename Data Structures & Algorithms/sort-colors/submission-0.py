class Solution:
    def sortColors(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        red_count = 0
        white_count = 0 
        blue_count = 0

        for num in nums: 
            if num == 0: 
                red_count += 1
            elif num == 1: 
                white_count += 1
            else: 
                blue_count += 1

        for i in range(len(nums)): 
            if i < red_count: 
                nums[i] = 0
            elif i < red_count + white_count: 
                nums[i] = 1
            else: 
                nums[i] = 2
            
        