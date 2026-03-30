class Solution:
    def containsNearbyDuplicate(self, nums: List[int], k: int) -> bool:
        window = set()
        n = len(nums)
        l = 0
        for r in range(n): 
            if r - l > k : 
                window.remove(nums[l])
                l += 1
            
            ele = nums[r]
            if ele in window: 
                return True
            
            window.add(ele)

        return False
        