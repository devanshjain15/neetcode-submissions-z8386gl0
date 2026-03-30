class Solution {
public:

    string encode(vector<string>& strs) {
        string result = ""; 
        for (int i = 0; i < strs.size(); i++ ) { 
            result.append(strs[i]); 
            result.append("#;"); 
        }
        return result; 
    }

    vector<string> decode(string s) {
        std::vector<string> result; 
        int n = s.size(); 
        int start = 0;
         
        for (int i = 0; i < n - 1; i++) { 
            if (s[i] == '#' && s[i + 1] == ';') { 
                result.push_back(s.substr(start, i - start)); 
                i++; 
                start = i + 1; 
            }
        }

        return result; 
    }
};
