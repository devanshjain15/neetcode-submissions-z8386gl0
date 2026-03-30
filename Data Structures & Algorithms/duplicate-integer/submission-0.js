class Solution {
    /**
     * @param {number[]} nums
     * @return {boolean}
     */
    hasDuplicate(nums) {
        const records = {}; 
        for(let i = 0; i < nums.length; i++){ 
            if (records[nums[i]] === undefined) { 
                records[nums[i]] = false; 
                continue; 
            }
            if (!records[nums[i]]) return true;
        }

        return false;
    }
}
