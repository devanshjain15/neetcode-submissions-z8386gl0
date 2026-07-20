class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        freq = defaultdict(int)
        res = nums[0]
        maxCount = 1

        for num in nums: 
            freq[num] += 1
            if maxCount < freq[num]: 
                res = num
                maxCount = freq[num]

        return res