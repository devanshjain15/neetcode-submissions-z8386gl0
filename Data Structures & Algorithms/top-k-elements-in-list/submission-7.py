import heapq

class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        freq = defaultdict(int)
        for num in nums: 
            freq[num] += 1

        # max heap pop k elements 
        result = []
        heap = [] 

        for num, f in freq.items(): 
            heapq.heappush(heap, (-f, num))

        for i in range(k): 
            _, num = heapq.heappop(heap)
            result.append(num)

        return result