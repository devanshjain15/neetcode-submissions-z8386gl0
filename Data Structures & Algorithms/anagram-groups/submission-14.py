class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        result = []
        isAppended = {}
        strCount = len(strs)
        for i in range(strCount):
            str1 = strs[i]
            if i not in isAppended: 
                result.append([str1])
            for j in range(strCount):
                str2 = strs[j] 
                if (j == i) or j in isAppended : continue  
                if isAnagram(str1, str2): 
                    result[len(result) - 1].append(str2)
                    isAppended[j] = True
            isAppended[i] = True
        
        return result
        
def isAnagram(str1, str2): 
    if (len(str2) != len(str1)): return False
    str1Record = charRecord(str1)
    str2Record = charRecord(str2)

    for c in str1Record: 
        if (c not in str2Record) or (c in str2Record and str2Record[c] != str1Record[c]) : return False

    return True

def charRecord(string): 
    record = {} 
    for c in string: 
        if c not in record: record[c] = 1 
        else: record[c] += 1
    return record
        