class Solution {
    /**
     * @param {string} s
     * @param {string} t
     * @return {boolean}
     */
    isAnagram(s, t) {
        if (s.length !== t.length) return false;

        const sRecord = new Map();

        for (let i = 0; i < s.length; i++) {
            if (sRecord.get(s[i])) sRecord.set(s[i], sRecord.get(s[i]) + 1);
            else sRecord.set(s[i], 1);
        }

        for (let i = 0; i < t.length; i++) {
            const char = t[i];
            if (!sRecord.get(char)) return false; 
            else sRecord.set(char, sRecord.get(char) - 1); 
        }

        for (let i = 0; i < s.length; i++) 
            if (sRecord.get(s[i]) !== 0) return false; 
        

        return true;
    }
}
