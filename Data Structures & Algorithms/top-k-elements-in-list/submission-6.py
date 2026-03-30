class BucketSort: 
    def __init__(self, size): 
        self.size = size
        self.buckets = [[] for _ in range(self.size + 1)]

    def append(self, index, value): 
        self.buckets[index].append(value)
        
    def getByIndex(self, index): 
        return self.buckets[index]

class Solution:
    def topKFrequent(self, nums, k):
        result = []
        distinctNums = []
        numsCount = len(nums)
        numsFreq = {}
        for i in range(numsCount):
            n = nums[i]
            if n not in numsFreq: 
                numsFreq[n] = 1
                distinctNums.append(n)
            else: numsFreq[n] += 1

        freq = numsFreq.values()
        maxNum = None
        for i in freq: 
            if not maxNum or i > maxNum: maxNum = i

        bucketSort = BucketSort(maxNum)
        for i in range(len(distinctNums)): 
            n = distinctNums[i]
            bucketSort.append(numsFreq[n], n)

        for i in range(bucketSort.size, -1, -1): 
            bucket = bucketSort.getByIndex(i)
            for j in bucket: 
                if(len(result) < k): result.append(j)
            if(len(result) == k) : break  

        return result