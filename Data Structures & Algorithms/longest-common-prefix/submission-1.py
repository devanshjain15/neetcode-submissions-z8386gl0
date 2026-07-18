class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        base = strs[0] 
        prefix_len = len(base)
        if prefix_len == 0: 
            return ""

        for s in strs:
            s_len = len(s)
            for j in range(prefix_len): 
                if j == s_len or base[j] != s[j]: 
                    prefix_len = j
                    break

        return base[:prefix_len]