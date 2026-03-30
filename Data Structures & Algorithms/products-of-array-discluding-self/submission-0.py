class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        result = []
        numLen = len(nums)
        for i in range(numLen): 
            product = 1
            for j in range(numLen): 
                if(j == i): continue
                product *= nums[j]
            result.append(product)
        return result

        
        