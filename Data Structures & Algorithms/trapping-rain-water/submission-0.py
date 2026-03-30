class Solution:
    def trap(self, nums: List[int]) -> int:
        import math
        last_idx = len(nums) - 1
        left_max = [0]
        right_max = [0]
        for i in range(1, last_idx + 1): 
            prev_left_ele = nums[i - 1]
            last_left_max_ele = left_max[i - 1]
            if prev_left_ele > last_left_max_ele: 
                left_max.append(prev_left_ele)
            else: 
                left_max.append(last_left_max_ele)

            prev_right_ele = nums[last_idx - i + 1]
            last_right_max_ele = right_max[i - 1]
            if prev_right_ele > last_right_max_ele: 
                right_max.append(prev_right_ele)
            else: 
                right_max.append(last_right_max_ele)
                
        count = 0
        for i, n in enumerate(nums):
                water_blocks = min(left_max[i], right_max[last_idx - i]) - n
                if water_blocks >= 1: 
                    count += water_blocks
        return count
        