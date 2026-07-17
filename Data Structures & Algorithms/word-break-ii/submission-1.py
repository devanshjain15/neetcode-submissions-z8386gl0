class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        word_set = set(wordDict)
        result = []
        def dfs(i: int, sentence: List[str]): 
            nonlocal result
            s_len = len(s)
            if i == s_len: 
                result.append(" ".join(sentence))
                return 

            for j in range(i, s_len): 
                sub_s = s[i : j + 1]
                if sub_s in word_set: 
                    sentence.append(sub_s)
                    dfs(j + 1, sentence)
                    sentence.pop()
        
        dfs(0, [])
        return result