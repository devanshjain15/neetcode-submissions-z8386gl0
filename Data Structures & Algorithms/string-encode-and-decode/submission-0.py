class Solution:

    def encode(self, strs: List[str]) -> str:
        result = ""
        for s in strs:
            strLen = len(s) 
            result = result + (s + ":;")
        return result

    def decode(self, s: str) -> List[str]:
        result = []
        temp = ""
        for i in range(len(s) - 1): 
            if(s[i] == ":" and s[i+1] == ";"): 
                result.append(temp)
                temp = ""
                continue
            if (s[i - 1] != ":"): temp += s[i]
        return result


