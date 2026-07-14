class Solution:
    def canPartitionKSubsets(self, nums: List[int], k: int) -> bool:
        total = sum(nums)
        if total % k != 0: 
            return False

        target = total // k
        states = [0 for _ in range(k)]
        nums.sort(reverse=True)
        if nums[0] > target: 
            return False

        return f(0, target, nums, states)


def f(i: int, target: int, nums: List[int], states: List[int]) -> bool: 
    if i == len(nums):
        return True

    for k in range(len(states)): 
        if states[k] + nums[i] > target: 
            continue
        states[k] += nums[i]
        if f(i + 1,  target, nums, states): 
            return True
        states[k] -= nums[i]

        if states[k] == 0: 
            break

    return False