class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        groups = defaultdict(list)
        for s in strs: 
            key = createKey(s)
            groups[key].append(s)

        return list(groups.values())

def createKey(s: str) -> str: 
    key = [0 for _ in range(26)]
    for c in s: 
        key[ord(c) - ord('a')] += 1

    return ",".join(map(str, key))