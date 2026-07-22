class Solution:
    def sortArray(self, nums: List[int]) -> List[int]:
        quickSort(0, len(nums) - 1, nums)
        return nums

def quickSort(start: int, end: int, nums: List[int]): 
    if start >= end: 
        return 

    p = partition(start, end, nums)

    quickSort(start, p, nums)
    quickSort(p + 1, end, nums)

def partition(start: int, end: int, nums: List[int]) -> int: 
    pivot = nums[(start + end)//2]
    i = start - 1
    j = end + 1
    while True: 
        i += 1
        while nums[i] < pivot: 
            i += 1

        j -= 1
        while nums[j] > pivot: 
            j -= 1
        
        if i >= j: 
            return j

        nums[i], nums[j] = nums[j], nums[i]

    
